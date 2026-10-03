use std::collections::BTreeMap;
use serde::{Deserialize, Serialize};
use crate::{Diagnostic, Emulator};

pub const ECIM_VERSION: u32 = 1;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Ecim {
    pub version: u32,
    pub source: Option<Emulator>,
    pub machine: Machine,
    pub storage: Storage,
    pub input: Input,
    pub audio: Audio,
    pub preserved: BTreeMap<String, String>,
    pub diagnostics: Vec<Diagnostic>,
}

impl Default for Ecim {
    fn default() -> Self {
        Self {
            version: ECIM_VERSION,
            source: None,
            machine: Machine::default(),
            storage: Storage::default(),
            input: Input::default(),
            audio: Audio::default(),
            preserved: BTreeMap::new(),
            diagnostics: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Machine {
    pub model: Option<String>,
    pub cpu: Option<String>,
    pub fpu: Option<String>,
    pub mmu: Option<bool>,
    pub jit: Option<bool>,
    pub chipset: Option<String>,
    pub video_standard: Option<String>,
    pub chip_ram_bytes: Option<u64>,
    pub slow_ram_bytes: Option<u64>,
    pub fast_ram_bytes: Option<u64>,
    pub z3_ram_bytes: Option<u64>,
    pub rom: Option<String>,
    pub rtg: Option<bool>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Storage {
    pub floppies: Vec<Option<String>>,
    pub hardfiles: Vec<String>,
    pub directories: Vec<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Input {
    pub joystick_port_0: Option<String>,
    pub joystick_port_1: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Audio {
    pub enabled: Option<bool>,
}
