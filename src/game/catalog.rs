#[derive(Debug, Clone)]
pub struct AreaCandidate {
    pub id: &'static str,
    pub min_level: u32,
    pub max_level: u32,
    pub base_exp: f64,
    pub base_gold: f64,
}

pub static HUNTS_CATALOG: &[AreaCandidate] = &[
        AreaCandidate { id: "esgoto_iniciante", min_level: 1, max_level: 20, base_exp: 105.0, base_gold: 55.0 },
        AreaCandidate { id: "pantano_sombrio", min_level: 20, max_level: 45, base_exp: 270.0, base_gold: 150.0 },
        AreaCandidate { id: "cemiterio_maldito", min_level: 45, max_level: 75, base_exp: 710.0, base_gold: 410.0 },
        AreaCandidate { id: "minas_de_ferro", min_level: 75, max_level: 115, base_exp: 1850.0, base_gold: 1150.0 },
        AreaCandidate { id: "ruinas_antigas", min_level: 115, max_level: 170, base_exp: 4500.0, base_gold: 2900.0 },
        AreaCandidate { id: "templo_do_dragao", min_level: 170, max_level: 240, base_exp: 11200.0, base_gold: 7400.0 },
        AreaCandidate { id: "torre_do_abismo", min_level: 240, max_level: 360, base_exp: 28000.0, base_gold: 19000.0 },
];

#[derive(Clone, Default)]
pub struct Catalog;

impl Catalog {
    pub fn select_best_hunt(&self, level: u32) -> &'static AreaCandidate {
        for area in HUNTS_CATALOG {
            if level >= area.min_level && level <= area.max_level {
                return area;
            }
        }
        &HUNTS_CATALOG[HUNTS_CATALOG.len() - 1]
    }
}
