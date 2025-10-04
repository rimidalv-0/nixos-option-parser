use serde_json::{self, Map, Value, from_str, json};
use std::env;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::PathBuf;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: {} <path-to-file> <path-to_dir>", &args[0]);
        std::process::exit(1);
    }

    let output_dir_path: PathBuf = PathBuf::from(&args[2]);

    Ok(())
}
