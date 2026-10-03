use crate::Emulator;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Detection {
    pub emulator: Emulator,
    pub confidence: u8,
    pub reason: &'static str,
}

pub fn detect_format(name: &str, contents: &str) -> Option<Detection> {
    let lower_name = name.to_ascii_lowercase();
    let lower = contents.to_ascii_lowercase();

    if lower.contains("config_description=") || lower.contains("amiberry") {
        return Some(Detection { emulator: Emulator::Amiberry, confidence: 80, reason: "Amiberry/UAE-style marker" });
    }
    if lower.contains("[fs-uae]") || lower.contains("fs-uae") {
        return Some(Detection { emulator: Emulator::FsUae, confidence: 90, reason: "FS-UAE marker" });
    }
    if lower_name.ends_with(".uae") || lower.contains("use_gui=") {
        return Some(Detection { emulator: Emulator::WinUae, confidence: 60, reason: "UAE configuration marker" });
    }
    if lower.contains("fellowng") {
        return Some(Detection { emulator: Emulator::FellowNg, confidence: 90, reason: "FellowNG marker" });
    }
    if lower.contains("copperline") {
        return Some(Detection { emulator: Emulator::Copperline, confidence: 90, reason: "Copperline marker" });
    }
    if lower.contains("fellow") {
        return Some(Detection { emulator: Emulator::Fellow, confidence: 70, reason: "Fellow marker" });
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_fs_uae_marker() {
        let d = detect_format("machine.conf", "[fs-uae]\nmodel = A1200").unwrap();
        assert_eq!(d.emulator, Emulator::FsUae);
    }

    #[test]
    fn detects_winuae_extension() {
        let d = detect_format("machine.uae", "cpu_type=68020").unwrap();
        assert_eq!(d.emulator, Emulator::WinUae);
    }
}
