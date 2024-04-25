use crate::binary_version::BinaryVersion;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};

const DEFAULT_NAME: &str = "<name>";
const DEFAULT_AUTHOR: &str = "<author>";
const DEFAULT_DESCRIPTION: &str = "<description>";

#[derive(Deserialize, Serialize)]
pub struct BinaryPatch {
    pub name: String,
    pub author: String,
    pub description: String,
    pub addresses: HashMap<BinaryVersion, BTreeMap<u64, u8>>,
}

impl Default for BinaryPatch {
    fn default() -> Self {
        Self {
            name: DEFAULT_NAME.to_string(),
            author: DEFAULT_AUTHOR.to_string(),
            description: DEFAULT_DESCRIPTION.to_string(),
            addresses: Default::default(),
        }
    }
}
