use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Ship {
    pub index: u32,
    pub armour: u32,
    pub cargo_capacity: u32,
    pub price: u32,
    pub primary_weapon_count: u32,
    pub secondary_weapon_count: u32,
    pub turret_count: u32,
    pub equipment_slot_count: u32,
    pub handling: u32,
}
