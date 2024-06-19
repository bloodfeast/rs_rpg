use crate::actors::player::PlayerStats;

#[derive(Debug, Eq, PartialEq)]
pub struct PlayerState {
    pub player_stats: PlayerStats,
    pub current_experience: u32,
    pub experience_to_next_level: u32,
}

fn calculate_experience_to_next_level(level: u32) -> u32 {
    match level {
        0..=19 => {
            // Use a smaller scaling factor or a different formula for levels below 20
            (50.0 * 2f32.powf(level as f32)).ceil() as u32
        }
        // Use a larger scaling factor or a different formula for levels above 20
        // we might want to use a different formula for levels above 20 to make the game more challenging
        _ => {
            (100.0 * 2f32.powf(level as f32)).ceil() as u32
        }
    }
}

impl PlayerState {
    pub fn new() -> Self {
        let player_stats = PlayerStats::new();

        PlayerState {
            player_stats,
            current_experience: 0,
            experience_to_next_level: 100,
        }
    }

    pub fn level_up(&mut self) {
        self.player_stats.level += 1;
        self.player_stats.level_up();
        self.experience_to_next_level = calculate_experience_to_next_level(self.player_stats.level);
    }

    pub fn gain_experience(&mut self, experience: u32) {
        self.current_experience += experience;

        if self.current_experience >= self.experience_to_next_level {
            self.level_up();
        }
    }
}