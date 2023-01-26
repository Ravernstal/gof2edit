use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Station {
    pub index: u32,
    pub name: String,
    pub system_index: u32,
    pub tech_level: u32,
    pub texture_index: u32,
}
