use crate::engine::circuit_breaker::{CircuitBreaker, CircuitBreakerConfig};
use crate::engine::stamina::{StaminaConfig, StaminaController};
use crate::game::types::{AccountConfig, CharacterState, InboundPacket};
use crate::network::socket::LuxSocketClient;
use crate::subsystem::dailies::DailyManager;
use crate::subsystem::inventory::InventoryManager;
use crate::subsystem::potions::PotionManager;
use serde_json::Value;
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, Ordering};
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{sleep, Duration};
use tracing::{debug, error, info, warn};

pub struct BotInstance {
    pub config: AccountConfig,
    pub circuit_breaker: Arc<CircuitBreaker>,
    pub stamina: Arc<StaminaController>,
    pub inventory: Arc<InventoryManager>,
    pub dailies: Arc<DailyManager>,
    pub potions: Arc<PotionManager>,
    pub socket: Arc<LuxSocketClient>,

    pub character: Arc<Mutex<CharacterState>>,
    pub kills: AtomicU64,
    pub gold: AtomicU64,
    pub exp: AtomicU64,
    pub current_level: AtomicU32,
    pub is_hunting: AtomicBool,
    pub running: AtomicBool,
}

impl BotInstance {
    pub fn new(config: AccountConfig) -> Self {
        let cb_cfg = CircuitBreakerConfig::default();
        let cb = Arc::new(CircuitBreaker::new(&config.id, cb_cfg));
        let stam = Arc::new(StaminaController::new(StaminaConfig::default()));

        let ws_url = if config.credentials.origin.is_empty() {
            "wss://luxidle.com.br/ws?livro=1&cacadas=1".to_string()
        } else {
            let orig = &config.credentials.origin;
            let scheme = if orig.starts_with("https") { "wss" } else { "ws" };
            let host = orig.trim_start_matches("https://").trim_start_matches("http://").trim_end_matches('/');
            format!("{}://{}/ws?livro=1&cacadas=1", scheme, host)
        };

        let socket = Arc::new(LuxSocketClient::new(&ws_url));

        Self {
            config,
            circuit_breaker: cb,
            stamina: stam,
            inventory: Arc::new(InventoryManager::default()),
            dailies: Arc::new(DailyManager::default()),
            potions: Arc::new(PotionManager::default()),
            socket,
            character: Arc::new(Mutex::new(CharacterState::default())),
            kills: AtomicU64::new(0),
            gold: AtomicU64::new(0),
            exp: AtomicU64::new(0),
            current_level: AtomicU32::new(1),
            is_hunting: AtomicBool::new(false),
            running: AtomicBool::new(false),
        }
    }

    pub async fn handle_message(&self, raw: &str) {
        let packet: InboundPacket = match serde_json::from_str(raw) {
            Ok(p) => p,
            Err(_) => return,
        };

        match packet.t.as_str() {
            "pong" => {}
            "account" => {
                info!("👤 Conta carregada no Lux Idle.");
                if let Some(chars) = packet.characters {
                    let chosen = if !self.config.credentials.character_name.is_empty() {
                        chars.iter().find(|c| c.name == self.config.credentials.character_name).cloned()
                    } else {
                        chars.first().cloned()
                    };

                    if let Some(c) = chosen {
                        info!("🎮 Selecionando personagem: {} (Nv. {})", c.name, c.level);
                        let _ = self.socket.send_json(&serde_json::json!({
                            "t": "selectChar",
                            "name": c.name
                        })).await;
                    }
                }
            }
            "welcome" | "state" => {
                if let Some(c) = packet.character {
                    if c.level > 0 {
                        self.current_level.store(c.level, Ordering::Relaxed);
                    }
                    if c.gold > 0 {
                        self.gold.store(c.gold as u64, Ordering::Relaxed);
                    }
                    *self.character.lock().await = c;
                }
            }
            "grupoConvite" => {
                if let Some(from) = packet.from {
                    info!("👥 Convite de party recebido de '{from}'. Aceitando...");
                    let _ = self.socket.send_json(&serde_json::json!({
                        "t": "grupo",
                        "action": "aceitar"
                    })).await;
                }
            }
            "coinMarket" => {
                debug!("💰 Cotação de Lux Coins atualizada.");
            }
            _ => {}
        }
    }

    pub async fn perform_automation_tick(&self) {
        if !self.socket.is_connected().await {
            return;
        }

        // 1. Auto-Hunt
        if self.config.settings.auto_hunt && !self.is_hunting.load(Ordering::Relaxed) {
            let target = if self.config.settings.hunt_target.is_empty() || self.config.settings.hunt_target == "auto" {
                let lvl = self.current_level.load(Ordering::Relaxed);
                if lvl < 20 { "hunt-rotworms" } else if lvl < 50 { "hunt-dragons" } else { "hunt-demons" }
            } else {
                &self.config.settings.hunt_target
            };

            info!("⚔️ Iniciando caçada em '{target}'...");
            let _ = self.socket.send_json(&serde_json::json!({
                "t": "startHunt",
                "hunt": target,
                "mode": "online",
                "distance": 0
            })).await;
            self.is_hunting.store(true, Ordering::Relaxed);
        }

        // 2. Bênçãos (Blessings)
        if self.config.settings.auto_bless {
            for b in 1..=5 {
                let _ = self.socket.send_json(&serde_json::json!({
                    "t": "buyBless",
                    "blessing": b
                })).await;
            }
        }

        // 3. Munição na Aljava (Quiver Paladino)
        if self.config.settings.auto_quiver {
            let _ = self.socket.send_json(&serde_json::json!({ "t": "quiverAuto" })).await;
        }

        // 4. Venda de loot
        if self.config.settings.auto_sell {
            let _ = self.socket.send_json(&serde_json::json!({ "t": "sellTrash" })).await;
        }

        // 5. Recompensa diária
        if self.config.settings.auto_daily {
            let _ = self.socket.send_json(&serde_json::json!({ "t": "claimDaily" })).await;
        }
    }

    pub async fn run_loop(self: Arc<Self>) {
        self.running.store(true, Ordering::SeqCst);
        info!("🚀 Bot [{}] (Lux Idle Online) conectado com protocolo nativo.", self.config.name);

        let this = Arc::clone(&self);
        let _ = self.socket.connect(move |raw| {
            let this = Arc::clone(&this);
            tokio::spawn(async move {
                this.handle_message(&raw).await;
            });
        }).await;

        // Handshake inicial
        let _ = self.socket.send_json(&serde_json::json!({ "t": "delta", "on": true, "sessao": true, "fundo": true })).await;
        let _ = self.socket.send_json(&serde_json::json!({ "t": "oculta", "on": false })).await;

        let token = &self.config.credentials.token;
        if !token.is_empty() && token != "seu-token-de-sessao-ou-jwt" {
            let _ = self.socket.send_json(&serde_json::json!({ "t": "resume", "token": token })).await;
        } else if !self.config.credentials.email.is_empty() {
            let _ = self.socket.send_json(&serde_json::json!({
                "t": "login",
                "email": self.config.credentials.email,
                "password": self.config.credentials.password
            })).await;
        }

        let mut ping_timer = tokio::time::interval(Duration::from_secs(20));
        let mut action_timer = tokio::time::interval(Duration::from_secs(3));

        while self.running.load(Ordering::Relaxed) {
            tokio::select! {
                _ = ping_timer.tick() => {
                    if self.socket.is_connected().await {
                        let _ = self.socket.send_json(&serde_json::json!({
                            "t": "ping",
                            "at": chrono::Utc::now().timestamp_millis()
                        })).await;
                    }
                }
                _ = action_timer.tick() => {
                    self.perform_automation_tick().await;
                }
            }
        }
    }
}
