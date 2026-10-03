use std::{env, fs, process};
use emuconf_core::{compatibility, detect_format, export, import, semantic_diff, Emulator};

fn usage() {
    eprintln!("Usage:");
    eprintln!("  emuconf detect <config>");
    eprintln!("  emuconf inspect <config>");
    eprintln!("  emuconf convert <config> --to <winuae|fs-uae|amiberry|fellow|fellowng|copperline>");
    eprintln!("  emuconf validate <config>");
    eprintln!("  emuconf compatibility <config> --to <format>");
    eprintln!("  emuconf diff <config-a> <config-b>");
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
        "validate" => {
            match import(detection.emulator,&contents) {
                Ok(_) => println!("valid: {}",detection.emulator.id()),
                Err(e) => { eprintln!("invalid: {e:?}"); process::exit(4); }
            }
        }
        "compatibility" => {
            if args.len()!=5 || args[3]!="--to" { usage(); process::exit(2); }
            let to=target(&args[4]).unwrap_or_else(|| { eprintln!("emuconf: unsupported target {}",args[4]); process::exit(2) });
            let ecim=import(detection.emulator,&contents).unwrap_or_else(|e| { eprintln!("emuconf: import failed: {e:?}"); process::exit(3) });
            let r=compatibility(&ecim,to);
            println!("{} -> {}: {}%",detection.emulator.id(),to.id(),r.score);
            for f in r.fields { println!("{:?}\t{}",f.fidelity,f.field); }
        }
        "diff" => {
            if args.len()!=4 { usage(); process::exit(2); }
            let other=fs::read_to_string(&args[3]).unwrap_or_else(|e| { eprintln!("emuconf: cannot read {}: {e}",args[3]); process::exit(1) });
            let od=detect_format(&args[3],&other).unwrap_or_else(|| { eprintln!("emuconf: second format not recognized"); process::exit(1) });
            let a=import(detection.emulator,&contents).unwrap_or_else(|e| { eprintln!("emuconf: import failed: {e:?}"); process::exit(3) });
            let b=import(od.emulator,&other).unwrap_or_else(|e| { eprintln!("emuconf: second import failed: {e:?}"); process::exit(3) });
            let d=semantic_diff(&a,&b);
            if d.is_empty(){println!("equivalent");}else{for field in d{println!("different: {field}");}}
        }
        "convert" => {
            if args.len()!=5 || args[3]!="--to" { usage(); process::exit(2); }
            let to=target(&args[4]).unwrap_or_else(|| { eprintln!("emuconf: unsupported target {}",args[4]); process::exit(2) });
            let ecim=import(detection.emulator,&contents).unwrap_or_else(|e| { eprintln!("emuconf: import failed: {e:?}"); process::exit(3) });
            let output=export(to,&ecim).unwrap_or_else(|e| { eprintln!("emuconf: export failed: {e:?}"); process::exit(3) });
            print!("{output}");
        }
        _ => { usage(); process::exit(2); }
    }
}
