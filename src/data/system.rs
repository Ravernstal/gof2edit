use crate::data::faction::Faction;
use crate::data::security_level::SecurityLevel;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct System {
    pub index: u32,
    pub name: String,
    pub security_level: SecurityLevel,
    pub faction: Faction,
    pub starts_unlocked: bool,
    pub map_coords: [u32; 3],
    pub jumpgate_station_id: Option<u32>,
    pub texture_index: u32,
    pub unknown_bytes: Vec<u32>,
    pub station_ids: Vec<u32>,
    pub linked_system_ids: Vec<u32>,
    pub footer_bytes: Vec<u32>,
}
