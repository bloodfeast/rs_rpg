use std::collections::HashMap;
use crate::actors::models::{ComputedAttributes, StatBlock};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum WeaponType {
    Axe,
    Sword,
    Club,
    Dagger,
    Bow,
    Fist,
    Polearm,
    Staff,
    Shield,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum EquipmentSlot {
    Head,
    Torso,
    Legs,
    Feet,
    Hands,
    Neck,
    RingOne,
    RingTwo,
    EarringOne,
    EarringTwo,
    MainHand(WeaponType),
    OffHand(WeaponType),
}

pub struct Equipment {
    pub slot: EquipmentSlot,
    pub name: String,
    pub description: String,
    pub required_stats: StatBlock,
    pub stat_modifiers: StatBlock,
    pub attribute_modifiers: ComputedAttributes,
}

pub struct Weapon {
    pub slot: EquipmentSlot,
    pub name: String,
    pub description: String,
    pub required_stats: StatBlock,
    pub stat_modifiers: StatBlock,
    pub attribute_modifiers: ComputedAttributes,
    pub weapon_type: WeaponType,
}

pub struct EquipmentManager {
    pub equipment: HashMap<EquipmentSlot, Equipment>
}

impl EquipmentManager {
    pub fn new() -> Self {
        EquipmentManager {
            equipment: HashMap::new()
        }
    }

    pub fn equip(&mut self, equipment: Equipment) {
        let slot = equipment.slot.clone();
        self.equipment.insert(slot, equipment);
    }

    pub fn unequip(&mut self, slot: EquipmentSlot) {
        self.equipment.remove(&slot);
    }

    pub fn get_equipment(&self, slot: EquipmentSlot) -> Option<&Equipment> {
        self.equipment.get(&slot)
    }
}