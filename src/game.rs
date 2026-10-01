#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Screen {
    Menu,
    Campaign,
    Army,
    Recruit,
}

pub enum GameCommand {
    ChangeScreen(Screen),
    InspectNext,
    PrepareBattle,
    ResetGame,
}

/// Tunable game rules live here, separate from state and rendering.
/// Later this can be deserialized from RON/JSON without changing the UI.
pub struct GameRules {
    pub target_simulation_hz: u16,
    pub army_bands: [(u32, u32); 5],
    pub espionage_cost: u32,
}

impl Default for GameRules {
    fn default() -> Self {
        Self {
            target_simulation_hz: 30,
            army_bands: [
                (1, 50),
                (51, 500),
                (501, 5_000),
                (5_001, 50_000),
                (50_001, 500_000),
            ],
            espionage_cost: 3,
        }
    }
}

pub struct Game {
    pub gold: u32,
    pub espionage: u32,
    pub soldiers: u32,
    pub stage: usize,
    pub intel_level: u8,
    pub notice: &'static str,
}

impl Game {
    pub fn new(_rules: &GameRules) -> Self {
        Self {
            gold: 180,
            espionage: 4,
            soldiers: 1,
            stage: 1,
            intel_level: 1,
            notice: "Your campaign begins at the edge of history.",
        }
    }

    pub fn army_level(&self, rules: &GameRules) -> usize {
        rules
            .army_bands
            .iter()
            .position(|(_, max)| self.soldiers <= *max)
            .unwrap_or(rules.army_bands.len() - 1)
            + 1
    }

    pub fn apply(&mut self, command: GameCommand, rules: &GameRules) {
        match command {
            GameCommand::ResetGame => *self = Self::new(rules),
            GameCommand::InspectNext if self.espionage >= rules.espionage_cost => {
                self.espionage -= rules.espionage_cost;
                self.intel_level = (self.intel_level + 1).min(3);
                self.notice = "New intelligence has been added to the enemy blocks.";
            }
            GameCommand::InspectNext => {
                self.notice = "Not enough espionage points.";
            }
            GameCommand::PrepareBattle => {
                self.notice = "Formation saved. Battle preparation is ready.";
            }
            GameCommand::ChangeScreen(_) => {}
        }
    }

    #[cfg(target_arch = "wasm32")]
    pub fn encode(&self) -> String {
        format!(
            "1|{}|{}|{}|{}|{}",
            self.gold, self.espionage, self.soldiers, self.stage, self.intel_level
        )
    }

    #[cfg(target_arch = "wasm32")]
    pub fn decode(value: &str) -> Option<Self> {
        let mut fields = value.split('|');
        if fields.next()? != "1" {
            return None;
        }
        Some(Self {
            gold: fields.next()?.parse().ok()?,
            espionage: fields.next()?.parse().ok()?,
            soldiers: fields.next()?.parse().ok()?,
            stage: fields.next()?.parse().ok()?,
            intel_level: fields.next()?.parse().ok()?,
            notice: "Saved campaign restored.",
        })
    }
}
