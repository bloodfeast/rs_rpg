use crate::actors::models::{ComputedAttributeEnum, ComputedAttributes, ComputedAttributeValues, StatBlock, StatsEnum, StatValues};

#[derive(Debug, Copy, Clone)]
pub struct PlayerStats {
    pub level: u32,
    pub base_stats: StatBlock,
    pub computed_attributes: ComputedAttributes
}

impl StatValues for PlayerStats {
    fn get_stat(&self, stat: StatsEnum) -> i32 {
        self.base_stats.get_stat(stat)
    }

    fn set_stat(&mut self, stat: StatsEnum, value: i32) {
        self.base_stats.set_stat(stat, value);
    }

    fn temp_modify_stat(&self, stat: StatsEnum, multiplier: f32) -> i32 {
        let base_stat = self.base_stats.get_stat(stat);

        (base_stat as f32 * multiplier).floor() as i32
    }
}

impl ComputedAttributeValues for PlayerStats {
    fn get_computed_attribute(&self, attribute: ComputedAttributeEnum) -> i32 {
        self.computed_attributes.get_computed_attribute(attribute)
    }

    fn set_computed_attribute(&mut self, attribute: ComputedAttributeEnum, value: i32) {
        self.computed_attributes.set_computed_attribute(attribute, value);
    }

    fn temp_modify_computed_attribute(&self, attribute: ComputedAttributeEnum, multiplier: f32) -> i32 {
        let base_attribute = self.computed_attributes.get_computed_attribute(attribute);

        (base_attribute as f32 * multiplier).floor() as i32
    }
}

impl PlayerStats {
    pub fn new() -> Self {
        let base_stats = StatBlock::default();
        PlayerStats {
            level: 1,
            base_stats,
            computed_attributes: ComputedAttributes::new(base_stats)
        }
    }

    fn set_passive_base_stat_increase(&mut self) {
        match &self.level {
            (level) if level % 2 == 0 => {
                self.base_stats.set_stat(StatsEnum::Wisdom, self.base_stats.get_stat(StatsEnum::Wisdom) + 1);
            },
            (level) if level % 3 == 0 => {
                self.base_stats.set_stat(StatsEnum::Constitution, self.base_stats.get_stat(StatsEnum::Strength) + 1);
            },
            _ => {}
        }
    }

    pub fn level_up(&mut self, stat_picked: StatsEnum) {
        self.level += 1;
        let stat_value = self.base_stats.get_stat(stat_picked);
        self.base_stats.set_stat(stat_picked, stat_value + 1);
        self.set_passive_base_stat_increase();

        self.computed_attributes = ComputedAttributes::new(self.base_stats);
    }
}
