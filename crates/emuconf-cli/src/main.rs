use std::{env, fs, process};
use emuconf_core::{detect_format, export, import, Emulator};

fn usage() {
    eprintln!("Usage:");
    eprintln!("  emuconf detect <config>");
    eprintln!("  emuconf inspect <config>");
    eprintln!("  emuconf convert <config> --to <winuae|fs-uae|amiberry|fellow|fellowng|copperline>");
}

fn target(s: &str) -> Option<Emulator> {
    match s.to_ascii_lowercase().as_str() {
        "winuae" => Some(Emulator::WinUae),
        "fs-uae" | "fsuae" => Some(Emulator::FsUae),
        "amiberry" => Some(Emulator::Amiberry),
        "fellow" => Some(Emulator::Fellow),
        "fellowng" | "fellow-ng" => Some(Emulator::FellowNg),
        "copperline" => Some(Emulator::Copperline),
        _ => None,
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 { usage(); process::exit(2); }
    let command=&args[1];
    let path=&args[2];
    let contents=fs::read_to_string(path).unwrap_or_else(|e| { eprintln!("emuconf: cannot read {path}: {e}"); process::exit(1) });
    let detection=detect_format(path,&contents).unwrap_or_else(|| { eprintln!("emuconf: format not recognized"); process::exit(1) });

    match command.as_str() {
        "detect" => println!("{} (confidence {}%): {}",detection.emulator.id(),detection.confidence,detection.reason),
        "inspect" => match import(detection.emulator,&contents) {
            Ok(e) => println!("{e:#?}"),
            Err(e) => { eprintln!("emuconf: cannot inspect: {e:?}"); process::exit(3); }
        },
        "convert" => {
            if args.len()!=5 || args[3]!="--to" { usage(); process::exit(2); }
            let to=target(&args[4]).unwrap_or_else(|| { eprintln!("emuconf: unsupported M1 target {}",args[4]); process::exit(2) });
            let ecim=import(detection.emulator,&contents).unwrap_or_else(|e| { eprintln!("emuconf: import failed: {e:?}"); process::exit(3) });
            let output=export(to,&ecim).unwrap_or_else(|e| { eprintln!("emuconf: export failed: {e:?}"); process::exit(3) });
            print!("{output}");
        }
        _ => { usage(); process::exit(2); }
    }
}
