use crate::actors::models::{ComputedAttributeEnum, ComputedAttributes, ComputedAttributeValues, StatBlock, StatsEnum, StatValues};

#[derive(Debug, Copy, Clone)]
pub struct BaseEnemyStats {
    pub level: u32,
    pub base_stats: StatBlock,
    pub computed_attributes: ComputedAttributes
}

impl StatValues for BaseEnemyStats {
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

impl ComputedAttributeValues for BaseEnemyStats {
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

impl BaseEnemyStats {
    pub fn new(level: u32, stats: Option<StatBlock>) -> Self {
        let base_stats = stats.unwrap_or_else(|| StatBlock::default());
        BaseEnemyStats {
            level,
            base_stats,
            computed_attributes: ComputedAttributes::new(base_stats)
        }
    }

    pub fn increase_stat(&mut self, stat: StatsEnum, value: i32) {
        self.base_stats.set_stat(stat, self.base_stats.get_stat(stat) + value);
        self.computed_attributes = ComputedAttributes::new(self.base_stats);
    }
}
