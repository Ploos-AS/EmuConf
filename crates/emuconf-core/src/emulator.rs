use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Emulator {
    WinUae,
    FsUae,
    Amiberry,
    Fellow,
    FellowNg,
    Copperline,
}

impl Emulator {
    pub const AMIGA_M0: [Self; 6] = [
        Self::WinUae,
        Self::FsUae,
        Self::Amiberry,
        Self::Fellow,
        Self::FellowNg,
        Self::Copperline,
    ];

    pub fn id(self) -> &'static str {
        match self {
            Self::WinUae => "winuae",
            Self::FsUae => "fs-uae",
            Self::Amiberry => "amiberry",
            Self::Fellow => "fellow",
            Self::FellowNg => "fellowng",
            Self::Copperline => "copperline",
        }
    }
}
