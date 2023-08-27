use clap::ValueEnum;
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum PatchTarget {
    /// stations.bin
    Stations,
    /// systems.bin
    Systems,
}

impl Display for PatchTarget {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Stations => write!(f, "stations"),
            Self::Systems => write!(f, "systems"),
        }
    }
}
