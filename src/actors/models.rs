#[derive(Debug, Clone, Copy)]
pub enum StatsEnum {
    Strength,
    Dexterity,
    Constitution,
    Intelligence,
    Wisdom,
    Charisma,
    Luck
}

#[derive(Debug, Clone, Copy)]
pub enum ComputedAttributeEnum {
    Health,
    Mana,
    Stamina,
    PhysicalResistance,
    PsychologicalResistance,
    MadnessLimit
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash,)]
pub struct StatBlock {
    strength: i32,
    dexterity: i32,
    constitution: i32,
    intelligence: i32,
    wisdom: i32,
    charisma: i32,
    luck: i32,
}

pub trait StatValues {
    fn get_stat(&self, stat: StatsEnum) -> i32;
    fn set_stat(&mut self, stat: StatsEnum, value: i32);
    fn temp_modify_stat(&self, stat: StatsEnum, multiplier: f32) -> i32;
}

impl StatBlock {
    pub fn new(custom_stat_block: StatBlock) -> Self {
        custom_stat_block
    }

    pub fn get_stat(&self, stat: StatsEnum) -> i32 {
        match stat {
            StatsEnum::Strength => self.strength,
            StatsEnum::Dexterity => self.dexterity,
            StatsEnum::Constitution => self.constitution,
            StatsEnum::Intelligence => self.intelligence,
            StatsEnum::Wisdom => self.wisdom,
            StatsEnum::Charisma => self.charisma,
            StatsEnum::Luck => self.luck,
        }
    }

    pub fn set_stat(&mut self, stat: StatsEnum, value: i32) {
        match stat {
            StatsEnum::Strength => self.strength = value,
            StatsEnum::Dexterity => self.dexterity = value,
            StatsEnum::Constitution => self.constitution = value,
            StatsEnum::Intelligence => self.intelligence = value,
            StatsEnum::Wisdom => self.wisdom = value,
            StatsEnum::Charisma => self.charisma = value,
            StatsEnum::Luck => self.luck = value,
        }
    }
}

impl Default for StatBlock {
    fn default() -> Self {
        Self {
            strength: 10,
            dexterity: 10,
            constitution: 10,
            intelligence: 10,
            wisdom: 10,
            charisma: 10,
            luck: 10,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash,)]
pub struct ComputedAttributes {
    health: i32,
    mana: i32,
    stamina: i32,
    physical_resistance: i32,
    psychological_resistance: i32,
    madness_limit: i32,
}

impl ComputedAttributes {
    pub fn new(stat_block: StatBlock) -> Self {
        let health = stat_block.get_stat(StatsEnum::Constitution)
            + (stat_block.get_stat(StatsEnum::Strength) * 10);
        let mana = stat_block.get_stat(StatsEnum::Intelligence)
            + (stat_block.get_stat(StatsEnum::Wisdom) * 10);
        let stamina = stat_block.get_stat(StatsEnum::Constitution)
            + (stat_block.get_stat(StatsEnum::Dexterity) * 5);
        let physical_resistance = stat_block.get_stat(StatsEnum::Strength)
            + (stat_block.get_stat(StatsEnum::Constitution) * 10);
        let psychological_resistance = stat_block.get_stat(StatsEnum::Constitution)
            + (stat_block.get_stat(StatsEnum::Wisdom) * 10);
        let madness_limit = (stat_block.get_stat(StatsEnum::Luck) * 10)
            + (stat_block.get_stat(StatsEnum::Intelligence) / 2);

        Self {
            health,
            mana,
            stamina,
            physical_resistance,
            psychological_resistance,
            madness_limit,
        }
    }

    pub fn get_computed_attribute(&self, attribute: ComputedAttributeEnum) -> i32 {
        match attribute {
            ComputedAttributeEnum::Health => self.health,
            ComputedAttributeEnum::Mana => self.mana,
            ComputedAttributeEnum::Stamina => self.stamina,
            ComputedAttributeEnum::PhysicalResistance => self.physical_resistance,
            ComputedAttributeEnum::PsychologicalResistance => self.psychological_resistance,
            ComputedAttributeEnum::MadnessLimit => self.madness_limit,
        }
    }

    pub fn set_computed_attribute(&mut self, attribute: ComputedAttributeEnum, value: i32) {
        match attribute {
            ComputedAttributeEnum::Health => self.health = value,
            ComputedAttributeEnum::Mana => self.mana = value,
            ComputedAttributeEnum::Stamina => self.stamina = value,
            ComputedAttributeEnum::PhysicalResistance => self.physical_resistance = value,
            ComputedAttributeEnum::PsychologicalResistance => self.psychological_resistance = value,
            ComputedAttributeEnum::MadnessLimit => self.madness_limit = value,
        }
    }
}

pub trait ComputedAttributeValues {
    fn get_computed_attribute(&self, attribute: ComputedAttributeEnum) -> i32;
    fn set_computed_attribute(&mut self, attribute: ComputedAttributeEnum, value: i32);
    fn temp_modify_computed_attribute(&self, attribute: ComputedAttributeEnum, multiplier: f32) -> i32;
}
