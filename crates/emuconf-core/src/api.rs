use crate::{compatibility, detect_format, export, import, AdapterError, CompatibilityReport, Ecim, Emulator};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ApiError {
    UnknownFormat,
    Adapter(AdapterError),
}

impl From<AdapterError> for ApiError {
    fn from(value: AdapterError) -> Self { Self::Adapter(value) }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoadedConfig {
    pub ecim: Ecim,
    pub detected: Emulator,
    pub confidence: u8,
}

impl LoadedConfig {
    pub fn compatibility(&self, target: Emulator) -> CompatibilityReport {
        compatibility(&self.ecim, target)
    }
    pub fn export(&self, target: Emulator) -> Result<String, ApiError> {
        Ok(export(target, &self.ecim)?)
    }
}

pub fn load(name: &str, contents: &str) -> Result<LoadedConfig, ApiError> {
    let detection=detect_format(name,contents).ok_or(ApiError::UnknownFormat)?;
    let ecim=import(detection.emulator,contents)?;
    Ok(LoadedConfig{ecim,detected:detection.emulator,confidence:detection.confidence})
}

pub fn load_as(source: Emulator, contents: &str) -> Result<LoadedConfig, ApiError> {
    let ecim=import(source,contents)?;
    Ok(LoadedConfig{ecim,detected:source,confidence:100})
}

pub fn convert(name: &str, contents: &str, target: Emulator) -> Result<String, ApiError> {
    load(name,contents)?.export(target)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn consumer_can_load_and_export_without_cli(){
        let cfg=load("machine.uae","cpu_type=68020\nchipset=aga\n").unwrap();
        assert_eq!(cfg.detected,Emulator::WinUae);
        assert_eq!(cfg.ecim.machine.cpu.as_deref(),Some("68020"));
        assert!(cfg.export(Emulator::FsUae).unwrap().starts_with("[fs-uae]"));
    }
    #[test] fn explicit_source_supports_embedded_consumers(){
        let cfg=load_as(Emulator::Copperline,"processor=68020\n").unwrap();
        assert_eq!(cfg.ecim.machine.cpu.as_deref(),Some("68020"));
        assert_eq!(cfg.confidence,100);
    }
}
