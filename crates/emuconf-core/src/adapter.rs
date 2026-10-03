use std::{collections::BTreeMap, error::Error, fmt};
use crate::{Ecim, Emulator};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AdapterError {
    UnsupportedSource(Emulator),
    UnsupportedTarget(Emulator),
}

impl fmt::Display for AdapterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedSource(e) => write!(f, "unsupported source format: {}", e.id()),
            Self::UnsupportedTarget(e) => write!(f, "unsupported target format: {}", e.id()),
        }
    }
}

impl Error for AdapterError {}

fn pairs(contents: &str) -> BTreeMap<String, String> {
    contents.lines().filter_map(|line| {
        let line=line.trim();
        if line.is_empty() || line.starts_with('#') || line.starts_with(';') || line.starts_with('[') { return None; }
        line.split_once('=').map(|(k,v)|(k.trim().to_ascii_lowercase(),v.trim().to_string()))
    }).collect()
}

fn bool_value(v: &str) -> Option<bool> {
    match v.trim().to_ascii_lowercase().as_str() {
        "1"|"true"|"yes"|"on" => Some(true),
        "0"|"false"|"no"|"off" => Some(false),
        _ => None,
    }
}
fn mib(v: &str) -> Option<u64> { v.parse::<u64>().ok().map(|n| n*1024*1024) }
fn first(p:&BTreeMap<String,String>, keys:&[&str])->Option<String> {
    keys.iter().find_map(|k|p.get(*k).cloned())
}
fn collect_indexed(p:&BTreeMap<String,String>, prefixes:&[&str])->Vec<String> {
    let mut v:Vec<(usize,String)>=Vec::new();
    for (k,value) in p {
        for prefix in prefixes {
            if let Some(rest)=k.strip_prefix(prefix) {
                if let Ok(i)=rest.parse::<usize>() { v.push((i,value.clone())); }
            }
        }
    }
    v.sort_by_key(|x|x.0);
    v.into_iter().map(|x|x.1).collect()
}

pub fn import(source: Emulator, contents: &str) -> Result<Ecim, AdapterError> {
    if !matches!(source, Emulator::WinUae|Emulator::FsUae|Emulator::Amiberry|Emulator::Fellow|Emulator::FellowNg|Emulator::Copperline) {
        return Err(AdapterError::UnsupportedSource(source));
    }
    let p=pairs(contents);
    let mut e=Ecim { source: Some(source), ..Ecim::default() };
    e.machine.model=first(&p,&["model","amiga_model","fellow_model","machine"]);
    e.machine.cpu=first(&p,&["cpu_type","cpu","fellow_cpu","processor"]);
    e.machine.fpu=first(&p,&["fpu_model","fpu"]);
    e.machine.mmu=first(&p,&["mmu","cpu_mmu"]).as_deref().and_then(bool_value);
    e.machine.jit=first(&p,&["jit","cachesize"]).as_deref().and_then(|v| if v=="0"{Some(false)} else {bool_value(v).or(Some(true))});
    e.machine.chipset=first(&p,&["chipset","chipset_compatible","fellow_chipset","custom_chipset"]);
    e.machine.video_standard=first(&p,&["video_standard"]).or_else(||p.get("ntsc").and_then(|v|bool_value(v)).map(|n|if n{"ntsc"}else{"pal"}.into()));
    e.machine.rom=first(&p,&["kickstart_rom_file","kickstart_file","kickstart","rom","rom_file"]);
    e.machine.chip_ram_bytes=first(&p,&["chip_memory","chipmem_size"]).as_deref().and_then(mib);
    e.machine.slow_ram_bytes=first(&p,&["slow_memory","bogomem_size"]).as_deref().and_then(mib);
    e.machine.fast_ram_bytes=first(&p,&["fast_memory","fastmem_size"]).as_deref().and_then(mib);
    e.machine.z3_ram_bytes=first(&p,&["zorro_iii_memory","z3mem_size"]).as_deref().and_then(mib);
    e.machine.rtg=first(&p,&["rtg","rtg_nocustom"]).as_deref().and_then(bool_value);
    e.audio.enabled=first(&p,&["sound","audio"]).as_deref().and_then(|v| if v=="none"||v=="0"{Some(false)}else{Some(true)});
    e.input.joystick_port_0=first(&p,&["joystick_port_0","joyport0"]);
    e.input.joystick_port_1=first(&p,&["joystick_port_1","joyport1"]);

    for i in 0..4 {
        let keys=[format!("floppy_drive_{i}"),format!("floppy{i}")];
        e.storage.floppies.push(keys.iter().find_map(|k|p.get(k).cloned()).filter(|s|!s.is_empty()));
    }
    e.storage.hardfiles=collect_indexed(&p,&["hard_drive_","hardfile"]);
    e.storage.directories=collect_indexed(&p,&["filesystem_","directory_"]);

    const KNOWN_PREFIXES:&[&str]=&[
        "model","amiga_model","fellow_model","cpu_type","cpu","fellow_cpu","fpu_model","fpu","mmu","cpu_mmu","jit","cachesize",
        "chipset","chipset_compatible","fellow_chipset","video_standard","ntsc","kickstart_rom_file","kickstart_file","kickstart","rom",
        "chip_memory","chipmem_size","slow_memory","bogomem_size","fast_memory","fastmem_size",
        "zorro_iii_memory","z3mem_size","rtg","rtg_nocustom","sound","audio","joystick_port_0",
        "joystick_port_1","joyport0","joyport1","floppy_drive_","floppy","hard_drive_","hardfile",
        "filesystem_","directory_","config_description"
    ];
    for (k,v) in p {
        if !KNOWN_PREFIXES.iter().any(|x|k==*x||k.starts_with(x)) { e.preserved.insert(k,v); }
    }
    Ok(e)
}

fn line(out:&mut String,key:&str,value:impl std::fmt::Display){out.push_str(&format!("{key}={value}\n"));}
fn bool_num(v:bool)->u8{if v{1}else{0}}

pub fn export(target: Emulator, ecim: &Ecim) -> Result<String, AdapterError> {
    if !matches!(target, Emulator::WinUae|Emulator::FsUae|Emulator::Amiberry|Emulator::Fellow|Emulator::FellowNg|Emulator::Copperline) {
        return Err(AdapterError::UnsupportedTarget(target));
    }
    let mut out=String::new();
    if target==Emulator::FsUae { out.push_str("[fs-uae]\n"); }
    if target==Emulator::Amiberry { line(&mut out,"config_description","Converted by EmuConf"); }
    if target==Emulator::Fellow { out.push_str("# Fellow configuration generated by EmuConf\n"); }
    if target==Emulator::FellowNg { out.push_str("# FellowNG configuration generated by EmuConf\n"); }
    if target==Emulator::Copperline { out.push_str("# Copperline configuration generated by EmuConf\n"); }
    if let Some(v)=&ecim.machine.model { line(&mut out,if matches!(target,Emulator::Fellow|Emulator::FellowNg|Emulator::Copperline){"fellow_model"}else{"model"},v); }
    if let Some(v)=&ecim.machine.cpu { line(&mut out,if matches!(target,Emulator::Fellow|Emulator::FellowNg|Emulator::Copperline){"fellow_cpu"}else{"cpu_type"},v); }
    if let Some(v)=&ecim.machine.fpu { line(&mut out,"fpu_model",v); }
    if let Some(v)=ecim.machine.mmu { line(&mut out,"mmu",bool_num(v)); }
    if let Some(v)=ecim.machine.jit { line(&mut out,"jit",bool_num(v)); }
    if let Some(v)=&ecim.machine.chipset { line(&mut out,if matches!(target,Emulator::Fellow|Emulator::FellowNg|Emulator::Copperline){"fellow_chipset"}else{"chipset"},v); }
    if let Some(v)=&ecim.machine.video_standard { line(&mut out,"video_standard",v); }
    if let Some(v)=&ecim.machine.rom { let key=if target==Emulator::FsUae{"kickstart_file"}else if matches!(target,Emulator::Fellow|Emulator::FellowNg|Emulator::Copperline){"kickstart"}else{"kickstart_rom_file"}; line(&mut out,key,v); }
    if let Some(v)=ecim.machine.chip_ram_bytes { line(&mut out,"chip_memory",v/(1024*1024)); }
    if let Some(v)=ecim.machine.slow_ram_bytes { line(&mut out,"slow_memory",v/(1024*1024)); }
    if let Some(v)=ecim.machine.fast_ram_bytes { line(&mut out,"fast_memory",v/(1024*1024)); }
    if let Some(v)=ecim.machine.z3_ram_bytes { line(&mut out,"zorro_iii_memory",v/(1024*1024)); }
    if let Some(v)=ecim.machine.rtg { line(&mut out,"rtg",bool_num(v)); }
    if let Some(v)=ecim.audio.enabled { line(&mut out,"sound",if v{"normal"}else{"none"}); }
    if let Some(v)=&ecim.input.joystick_port_0 { line(&mut out,"joystick_port_0",v); }
    if let Some(v)=&ecim.input.joystick_port_1 { line(&mut out,"joystick_port_1",v); }
    for (i,v) in ecim.storage.floppies.iter().enumerate() { if let Some(v)=v { line(&mut out,&format!("floppy_drive_{i}"),v); } }
    for (i,v) in ecim.storage.hardfiles.iter().enumerate() { line(&mut out,&format!("hard_drive_{i}"),v); }
    for (i,v) in ecim.storage.directories.iter().enumerate() { line(&mut out,&format!("filesystem_{i}"),v); }
    for (k,v) in &ecim.preserved { line(&mut out,k,v); }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    const FULL:&str="model=A1200\ncpu_type=68020\nfpu_model=68882\nmmu=1\njit=0\nchipset=aga\nvideo_standard=pal\nchip_memory=2\nslow_memory=1\nfast_memory=8\nzorro_iii_memory=16\nkickstart_rom_file=kick.rom\nrtg=1\nsound=normal\njoystick_port_0=mouse\njoystick_port_1=joy0\nfloppy_drive_0=Workbench.adf\nhard_drive_0=system.hdf\nfilesystem_0=DH1:/data\n";

    #[test]
    fn winuae_to_fsuae_round_trip_machine() {
        let ecim=import(Emulator::WinUae,FULL).unwrap();
        let fs=export(Emulator::FsUae,&ecim).unwrap();
        let again=import(Emulator::FsUae,&fs).unwrap();
        assert_eq!(ecim.machine,again.machine);
        assert_eq!(ecim.storage,again.storage);
        assert_eq!(ecim.input,again.input);
        assert_eq!(ecim.audio,again.audio);
    }

    #[test]
    fn amiberry_round_trip() {
        let ecim=import(Emulator::Amiberry,&format!("config_description=test\n{FULL}")).unwrap();
        let text=export(Emulator::Amiberry,&ecim).unwrap();
        let again=import(Emulator::Amiberry,&text).unwrap();
        assert_eq!(ecim.machine,again.machine);
    }

    #[test]
    fn fellowng_round_trip_core() {
        let src="# FellowNG\nfellow_model=A500\nfellow_cpu=68000\nfellow_chipset=ocs\nkickstart=kick13.rom\nchip_memory=1\nfloppy_drive_0=game.adf\n";
        let ecim=import(Emulator::FellowNg,src).unwrap();
        let out=export(Emulator::FellowNg,&ecim).unwrap();
        let again=import(Emulator::FellowNg,&out).unwrap();
        assert_eq!(ecim.machine,again.machine);
        assert_eq!(ecim.storage,again.storage);
    }

    #[test]
    fn unknown_fields_are_preserved() {
        let ecim=import(Emulator::Amiberry,"config_description=x\nfoo=bar\n").unwrap();
        assert_eq!(ecim.preserved.get("foo").map(String::as_str),Some("bar"));
    }
}
