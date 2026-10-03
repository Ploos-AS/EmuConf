use std::collections::BTreeMap;
use serde::{Deserialize, Serialize};
use crate::{Diagnostic, Emulator};

pub const ECIM_VERSION: u32 = 0;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Ecim {
    pub version: u32,
    pub source: Option<Emulator>,
    pub machine: Machine,
    pub preserved: BTreeMap<String, String>,
    pub diagnostics: Vec<Diagnostic>,
}

impl Default for Ecim {
    fn default() -> Self {
        Self {
            version: ECIM_VERSION,
            source: None,
            machine: Machine::default(),
            preserved: BTreeMap::new(),
            diagnostics: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Machine {
    pub cpu: Option<String>,
    pub chipset: Option<String>,
    pub video_standard: Option<String>,
    pub chip_ram_bytes: Option<u64>,
    pub fast_ram_bytes: Option<u64>,
    pub rom: Option<String>,
}
