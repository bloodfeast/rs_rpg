use crate::actors::models::{ComputedAttributes, StatBlock};
use crate::equipment::equipment_manager::EquipmentSlot;

pub struct SacrificialRobes {
    pub slot: EquipmentSlot,
    pub name: String,
    pub description: String,
    pub required_stats: StatBlock,
    pub stat_modifiers: StatBlock,
    pub attribute_modifiers: ComputedAttributes,
}

impl SacrificialRobes {
    pub fn new() -> Self {
        SacrificialRobes {
            slot: EquipmentSlot::Torso,
            name: String::from("Sacrificial Robes"),
            description: String::from("Robes that are used in sacrificial rituals."),
            required_stats: StatBlock::default(),
            stat_modifiers: StatBlock::default(),
            attribute_modifiers: ComputedAttributes {
                health: 0,
                mana: 0,
                stamina: 0,
                physical_resistance: 0,
                psychological_resistance: 0,
                madness_limit: 0,
            },
        }
    }

    pub fn new_with_modifiers(stat_modifiers: StatBlock, attribute_modifiers: ComputedAttributes) -> Self {
        SacrificialRobes {
            slot: EquipmentSlot::Torso,
            name: String::from("Sacrificial Robes"),
            description: String::from("Robes that are used in sacrificial rituals."),
            required_stats: StatBlock::default(),
            stat_modifiers,
            attribute_modifiers,
        }
    }
}