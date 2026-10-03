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

    if lower.contains("[fs-uae]") || lower_name.ends_with(".fs-uae") {
        return Some(Detection {
            emulator: Emulator::FsUae,
            confidence: 95,
            reason: "FS-UAE section or extension",
        });
    }
    if lower.contains("amiberry") || lower.contains("config_description=amiberry") {
        return Some(Detection {
            emulator: Emulator::Amiberry,
            confidence: 95,
            reason: "Amiberry marker",
        });
    }
    if lower.contains("fellowng") {
        return Some(Detection {
            emulator: Emulator::FellowNg,
            confidence: 90,
            reason: "FellowNG marker",
        });
    }
    if lower.contains("copperline") {
        return Some(Detection {
            emulator: Emulator::Copperline,
            confidence: 90,
            reason: "Copperline marker",
        });
    }
    if lower.contains("fellow") {
        return Some(Detection {
            emulator: Emulator::Fellow,
            confidence: 70,
            reason: "Fellow marker",
        });
    }
    if lower_name.ends_with(".uae") || lower.contains("use_gui=") || lower.contains("cpu_type=") {
        return Some(Detection {
            emulator: Emulator::WinUae,
            confidence: 60,
            reason: "UAE-style configuration",
        });
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_fs_uae() {
        assert_eq!(
            detect_format("x.fs-uae", "[fs-uae]\nmodel=A1200")
                .unwrap()
                .emulator,
            Emulator::FsUae
        );
    }

    #[test]
    fn generic_uae_is_winuae() {
        assert_eq!(
            detect_format("x.uae", "cpu_type=68020").unwrap().emulator,
            Emulator::WinUae
        );
    }

    #[test]
    fn explicit_amiberry_wins() {
        assert_eq!(
            detect_format("x.uae", "config_description=Amiberry profile")
                .unwrap()
                .emulator,
            Emulator::Amiberry
        );
    }
}
