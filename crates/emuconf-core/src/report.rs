use crate::{Ecim, Emulator, Fidelity};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldReport {
    pub field: &'static str,
    pub fidelity: Fidelity,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompatibilityReport {
    pub target: Emulator,
    pub score: u8,
    pub fields: Vec<FieldReport>,
}

fn supported(target: Emulator, field: &'static str) -> Fidelity {
    match target {
        Emulator::WinUae|Emulator::FsUae|Emulator::Amiberry => Fidelity::Exact,
        Emulator::Fellow => match field {
            "machine.fpu"|"machine.mmu"|"machine.jit"|"machine.z3_ram"|"machine.rtg" => Fidelity::Unsupported,
            _ => Fidelity::Mapped,
        },
        Emulator::FellowNg => match field {
            "machine.jit"|"machine.z3_ram"|"machine.rtg" => Fidelity::Approximate,
            _ => Fidelity::Mapped,
        },
        Emulator::Copperline => Fidelity::Mapped,
    }
}

pub fn compatibility(ecim: &Ecim, target: Emulator) -> CompatibilityReport {
    let mut fields=Vec::new();
    macro_rules! add {($present:expr,$name:literal)=>{if $present {fields.push(FieldReport{field:$name,fidelity:supported(target,$name)});}}}
    add!(ecim.machine.model.is_some(),"machine.model");
    add!(ecim.machine.cpu.is_some(),"machine.cpu");
    add!(ecim.machine.fpu.is_some(),"machine.fpu");
    add!(ecim.machine.mmu.is_some(),"machine.mmu");
    add!(ecim.machine.jit.is_some(),"machine.jit");
    add!(ecim.machine.chipset.is_some(),"machine.chipset");
    add!(ecim.machine.video_standard.is_some(),"machine.video_standard");
    add!(ecim.machine.chip_ram_bytes.is_some(),"machine.chip_ram");
    add!(ecim.machine.slow_ram_bytes.is_some(),"machine.slow_ram");
    add!(ecim.machine.fast_ram_bytes.is_some(),"machine.fast_ram");
    add!(ecim.machine.z3_ram_bytes.is_some(),"machine.z3_ram");
    add!(ecim.machine.rom.is_some(),"machine.rom");
    add!(ecim.machine.rtg.is_some(),"machine.rtg");
    add!(ecim.audio.enabled.is_some(),"audio.enabled");
    add!(ecim.input.joystick_port_0.is_some(),"input.port0");
    add!(ecim.input.joystick_port_1.is_some(),"input.port1");
    add!(ecim.storage.floppies.iter().any(Option::is_some),"storage.floppies");
    add!(!ecim.storage.hardfiles.is_empty(),"storage.hardfiles");
    add!(!ecim.storage.directories.is_empty(),"storage.directories");
    for _ in &ecim.preserved { fields.push(FieldReport{field:"preserved.extension",fidelity:Fidelity::Preserved}); }
    let total=fields.len();
    let points:usize=fields.iter().map(|f|match f.fidelity{Fidelity::Exact=>100,Fidelity::Mapped=>95,Fidelity::Preserved=>90,Fidelity::Approximate=>50,Fidelity::Unsupported=>0}).sum();
    CompatibilityReport{target,score:if total==0{100}else{(points/total) as u8},fields}
}

pub fn semantic_diff(a:&Ecim,b:&Ecim)->Vec<&'static str>{
    let mut d=Vec::new();
    if a.machine!=b.machine{d.push("machine");}
    if a.storage!=b.storage{d.push("storage");}
    if a.input!=b.input{d.push("input");}
    if a.audio!=b.audio{d.push("audio");}
    if a.preserved!=b.preserved{d.push("preserved");}
    d
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn fellow_reports_unsupported_advanced_features(){
        let mut e=Ecim::default(); e.machine.jit=Some(true); e.machine.rtg=Some(true);
        let r=compatibility(&e,Emulator::Fellow);
        assert!(r.score<100);
        assert!(r.fields.iter().all(|f|f.fidelity==Fidelity::Unsupported));
    }
    #[test] fn diff_ignores_source_metadata(){
        let mut a=Ecim::default(); let mut b=a.clone(); b.source=Some(Emulator::WinUae);
        assert!(semantic_diff(&a,&b).is_empty()); a.machine.cpu=Some("68020".into());
        assert_eq!(semantic_diff(&a,&b),vec!["machine"]);
    }
}
