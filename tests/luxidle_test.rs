use bot_luxidle_rust::game::types::*;
use serde_json::json;

#[test]
fn test_luxidle_handshake_serialization() {
    let delta = json!({
        "t": "delta",
        "on": true,
        "sessao": true,
        "fundo": true
    });
    assert_eq!(delta["t"], "delta");
    assert_eq!(delta["sessao"], true);

    let resume = json!({
        "t": "resume",
        "token": "test-jwt-lux"
    });
    assert_eq!(resume["token"], "test-jwt-lux");
}

#[test]
fn test_luxidle_inbound_packets() {
    let raw = json!({
        "t": "account",
        "characters": [
            {
                "name": "Knight Master",
                "level": 45,
                "vocation": "Elite Knight"
            },
            {
                "name": "Sorcerer Lux",
                "level": 20,
                "vocation": "Master Sorcerer"
            }
        ]
    });

    let p: InboundPacket = serde_json::from_value(raw).unwrap();
    assert_eq!(p.t, "account");
    let chars = p.characters.unwrap();
    assert_eq!(chars.len(), 2);
    assert_eq!(chars[0].name, "Knight Master");
    assert_eq!(chars[0].level, 45);
}

#[test]
fn test_luxidle_state_update() {
    let raw = json!({
        "t": "state",
        "character": {
            "name": "Knight Master",
            "level": 50,
            "vocation": "Elite Knight",
            "hp": 950,
            "mana": 280,
            "gold": 125000,
            "coins": 45
        }
    });

    let p: InboundPacket = serde_json::from_value(raw).unwrap();
    let char_state = p.character.unwrap();
    assert_eq!(char_state.level, 50);
    assert_eq!(char_state.gold, 125000);
    assert_eq!(char_state.coins, 45);
}
