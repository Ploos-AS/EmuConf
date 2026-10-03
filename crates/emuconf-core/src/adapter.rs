use std::collections::BTreeMap;
use crate::{Ecim, Emulator};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AdapterError {
    UnsupportedSource(Emulator),
    UnsupportedTarget(Emulator),
}

fn pairs(contents: &str) -> BTreeMap<String, String> {
    contents.lines().filter_map(|line| {
        let line=line.trim();
        if line.is_empty() || line.starts_with('#') || line.starts_with(';') || line.starts_with('[') { return None; }
        line.split_once('=').map(|(k,v)|(k.trim().to_ascii_lowercase(),v.trim().to_string()))
    }).collect()
}

fn mib(v: &str) -> Option<u64> { v.parse::<u64>().ok().map(|n| n*1024*1024) }

pub fn import(source: Emulator, contents: &str) -> Result<Ecim, AdapterError> {
    if !matches!(source, Emulator::WinUae|Emulator::FsUae|Emulator::Amiberry) {
        return Err(AdapterError::UnsupportedSource(source));
    }
    let p=pairs(contents);
    let mut e=Ecim { source: Some(source), ..Ecim::default() };
    e.machine.cpu=p.get("cpu_type").or_else(||p.get("cpu")).cloned();
    e.machine.chipset=p.get("chipset").cloned();
    e.machine.video_standard=p.get("video_standard").cloned();
    e.machine.rom=p.get("kickstart_rom_file").or_else(||p.get("kickstart_file")).cloned();
    e.machine.chip_ram_bytes=p.get("chip_memory").and_then(|v|mib(v));
    e.machine.fast_ram_bytes=p.get("fast_memory").and_then(|v|mib(v));

    const KNOWN: &[&str]=&["cpu_type","cpu","chipset","video_standard","kickstart_rom_file","kickstart_file","chip_memory","fast_memory","model","config_description"];
    for (k,v) in p {
        if !KNOWN.contains(&k.as_str()) { e.preserved.insert(k,v); }
    }
    Ok(e)
}

pub fn export(target: Emulator, ecim: &Ecim) -> Result<String, AdapterError> {
    if !matches!(target, Emulator::WinUae|Emulator::FsUae|Emulator::Amiberry) {
        return Err(AdapterError::UnsupportedTarget(target));
    }
    let mut out=String::new();
    if target==Emulator::FsUae { out.push_str("[fs-uae]\n"); }
    if target==Emulator::Amiberry { out.push_str("config_description=Converted by EmuConf\n"); }
    if let Some(v)=&ecim.machine.cpu { out.push_str(&format!("cpu_type={v}\n")); }
    if let Some(v)=&ecim.machine.chipset { out.push_str(&format!("chipset={v}\n")); }
    if let Some(v)=&ecim.machine.video_standard { out.push_str(&format!("video_standard={v}\n")); }
    if let Some(v)=&ecim.machine.rom {
        let key=if target==Emulator::FsUae {"kickstart_file"} else {"kickstart_rom_file"};
        out.push_str(&format!("{key}={v}\n"));
    }
    if let Some(v)=ecim.machine.chip_ram_bytes { out.push_str(&format!("chip_memory={}\n",v/(1024*1024))); }
    if let Some(v)=ecim.machine.fast_ram_bytes { out.push_str(&format!("fast_memory={}\n",v/(1024*1024))); }
    for (k,v) in &ecim.preserved { out.push_str(&format!("{k}={v}\n")); }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn winuae_to_fsuae_round_trip_core_fields() {
        let src="cpu_type=68020\nchipset=aga\nchip_memory=2\nfast_memory=8\nkickstart_rom_file=kick.rom\n";
        let ecim=import(Emulator::WinUae,src).unwrap();
        let fs=export(Emulator::FsUae,&ecim).unwrap();
        let again=import(Emulator::FsUae,&fs).unwrap();
        assert_eq!(ecim.machine,again.machine);
    }

    #[test]
    fn unknown_fields_are_preserved() {
        let ecim=import(Emulator::Amiberry,"config_description=x\nfoo=bar\n").unwrap();
        assert_eq!(ecim.preserved.get("foo").map(String::as_str),Some("bar"));
    }
}
