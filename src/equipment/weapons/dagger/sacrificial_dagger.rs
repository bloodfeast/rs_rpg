use crate::actors::models::{ComputedAttributes, StatBlock};
use crate::equipment::equipment_manager::{EquipmentSlot, WeaponType};

pub struct SacrificialDagger {
    pub slot: EquipmentSlot,
    pub name: String,
    pub description: String,
    pub required_stats: StatBlock,
    pub stat_modifiers: StatBlock,
    pub attribute_modifiers: ComputedAttributes,
    pub weapon_type: WeaponType,
}

impl SacrificialDagger {
    pub fn new() -> Self {
        SacrificialDagger {
            slot: EquipmentSlot::MainHand(WeaponType::Dagger),
            name: String::from("Sacrificial Dagger"),
            description: String::from("A dagger that is used in sacrificial rituals."),
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
            weapon_type: WeaponType::Dagger,
        }
    }

    pub fn new_with_modifiers(stat_modifiers: StatBlock, attribute_modifiers: ComputedAttributes) -> Self {
        SacrificialDagger {
            slot: EquipmentSlot::MainHand(WeaponType::Dagger),
            name: String::from("Sacrificial Dagger"),
            description: String::from("A dagger that is used in sacrificial rituals."),
            required_stats: StatBlock::default(),
            stat_modifiers,
            attribute_modifiers,
            weapon_type: WeaponType::Dagger,
        }
    }
}