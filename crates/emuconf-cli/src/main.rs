use std::{env, fs, process};
use emuconf_core::detect_format;

fn usage() {
    eprintln!("Usage:");
    eprintln!("  emuconf detect <config>");
    eprintln!("  emuconf inspect <config>");
    eprintln!("  emuconf convert <config> --to <format>");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        usage();
        process::exit(2);
    }

    let command = args[1].as_str();
    let path = &args[2];
    let contents = match fs::read_to_string(path) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("emuconf: cannot read {path}: {e}");
            process::exit(1);
        }
    };

    match command {
        "detect" | "inspect" => match detect_format(path, &contents) {
            Some(d) => println!("{} (confidence {}%): {}", d.emulator.id(), d.confidence, d.reason),
            None => {
                eprintln!("emuconf: format not recognized");
                process::exit(1);
            }
        },
        "convert" => {
            eprintln!("emuconf: conversion adapters begin in M1");
            process::exit(3);
        }
        _ => {
            usage();
            process::exit(2);
        }
    }
}
