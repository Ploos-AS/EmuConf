use emuconf_core::{load, Emulator};

fn main() {
    let source="cpu_type=68020\nchipset=aga\nchip_memory=2\n";
    let config=load("import.uae",source).expect("configuration should load");

    println!("detected: {}",config.detected.id());
    println!("ECIM: {:#?}",config.ecim);

    let report=config.compatibility(Emulator::FsUae);
    println!("FS-UAE compatibility: {}%",report.score);

    let converted=config.export(Emulator::FsUae).expect("export should succeed");
    println!("{converted}");
}
