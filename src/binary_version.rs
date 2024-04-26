use clap::ValueEnum;
use serde::{Deserialize, Serialize};

#[derive(
    Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, ValueEnum,
)]
pub enum BinaryVersion {
    /// libgof2hdaa.so
    Android,
    /// Galaxy_on_Fire_2_HD
    Ios,
}
