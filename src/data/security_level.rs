use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum SecurityLevel {
    Dangerous,
    Risky,
    Average,
    Secure,
}

impl SecurityLevel {
    pub fn from_u32(x: u32) -> Option<Self> {
        match x {
            0 => Some(Self::Dangerous),
            1 => Some(Self::Risky),
            2 => Some(Self::Average),
            3 => Some(Self::Secure),
            _ => None,
        }
    }

    pub fn code(&self) -> u32 {
        match self {
            Self::Dangerous => 0,
            Self::Risky => 1,
            Self::Average => 2,
            Self::Secure => 3,
        }
    }
}
