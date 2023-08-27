use clap::ValueEnum;
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum UnpackTarget {
    /// items.bin
    Items,
    /// names_*.bin
    Lang,
    /// ships.bin
    Ships,
    /// stations.bin
    Stations,
    /// systems.bin
    Systems,
}

impl Display for UnpackTarget {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Items => write!(f, "items"),
            Self::Lang => write!(f, "lang strings"),
            Self::Ships => write!(f, "ships"),
            Self::Stations => write!(f, "stations"),
            Self::Systems => write!(f, "systems"),
        }
    }
}
