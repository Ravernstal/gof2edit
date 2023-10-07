use clap::ValueEnum;
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum RepackTarget {
    /// items.bin
    Items,
    /// *.lang
    Lang,
    /// gof2_save_game_preview_*
    SavePreview,
    /// ships.bin
    Ships,
    /// weapons_*.bin
    ShipPositions,
    /// stations.bin
    Stations,
    /// systems.bin
    Systems,
    /// wanted.bin
    Wanted,
}

impl Display for RepackTarget {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Items => write!(f, "items"),
            Self::Lang => write!(f, "lang strings"),
            Self::SavePreview => write!(f, "save preview"),
            Self::Ships => write!(f, "ships"),
            Self::ShipPositions => write!(f, "ship positions"),
            Self::Stations => write!(f, "stations"),
            Self::Systems => write!(f, "systems"),
            Self::Wanted => write!(f, "most wanted"),
        }
    }
}
