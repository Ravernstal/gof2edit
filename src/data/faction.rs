use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum Faction {
    Terran,
    Vossk,
    Nivelian,
    Midorian,
}

impl Faction {
    pub fn from_u32(x: u32) -> Option<Self> {
        match x {
            0 => Some(Self::Terran),
            1 => Some(Self::Vossk),
            2 => Some(Self::Nivelian),
            3 => Some(Self::Midorian),
            _ => None,
        }
    }

    pub fn code(&self) -> u32 {
        match self {
            Self::Terran => 0,
            Self::Vossk => 1,
            Self::Nivelian => 2,
            Self::Midorian => 3,
        }
    }
}
