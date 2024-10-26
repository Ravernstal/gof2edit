use crate::binary_version::BinaryVersion;
use serde::{Deserialize, Serialize};
use serde_hex::{CompactPfx, SerHex};
use std::collections::BTreeMap;

const DEFAULT_NAME: &str = "<name>";
const DEFAULT_AUTHOR: &str = "<author>";
const DEFAULT_DESCRIPTION: &str = "<description>";

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(transparent)]
pub struct Address(#[serde(with = "SerHex::<CompactPfx>")] u64);

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct Value(#[serde(with = "SerHex::<CompactPfx>")] u8);

#[derive(Debug, Deserialize, Serialize)]
pub struct BinaryPatch {
    pub name: String,
    pub author: String,
    pub description: String,
    pub addresses: BTreeMap<BinaryVersion, BTreeMap<Address, Value>>,
}

impl Address {
    pub fn new(value: u64) -> Self {
        Self(value)
    }

    pub fn get(self) -> u64 {
        self.0
    }
}

impl Value {
    pub fn new(value: u8) -> Self {
        Self(value)
    }

    pub fn get(self) -> u8 {
        self.0
    }
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
