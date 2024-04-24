use crate::binary_version::BinaryVersion;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize)]
pub struct BinaryPatch {
    name: String,
    description: String,
    addresses: HashMap<BinaryVersion, HashMap<usize, u8>>,
}
