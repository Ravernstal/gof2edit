use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct LangString {
    pub index: u32,
    pub length: u16,
    pub string: String,
}
