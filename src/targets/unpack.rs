use clap::ValueEnum;
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum UnpackTarget {
    /// agents.bin
    Agents,
    /// items.bin
    Items,
    /// *.lang
    Lang,
    /// ticker.bin
    NewsItems,
    /// gof2_save_game_*
    Save,
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

impl Display for UnpackTarget {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Agents => write!(f, "agents"),
            Self::Items => write!(f, "items"),
            Self::Lang => write!(f, "lang strings"),
            Self::NewsItems => write!(f, "news items"),
            Self::Save => write!(f, "save game"),
            Self::SavePreview => write!(f, "save preview"),
            Self::Ships => write!(f, "ships"),
            Self::ShipPositions => write!(f, "ship positions"),
            Self::Stations => write!(f, "stations"),
            Self::Systems => write!(f, "systems"),
            Self::Wanted => write!(f, "most wanted"),
        }
    }
}
