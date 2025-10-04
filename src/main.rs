use serde_json::{self, Map, Value};
use std::env;
use std::fs::{self, File, OpenOptions};
use std::io::{self, Write};
use std::path::PathBuf;

fn main() -> io::Result<()> {
	let args: Vec<String> = env::args().collect();
	if args.len() != 3 {
		eprintln!(
			"Usage: {} <path-to-file || json-object> <path-to_dir>",
			&args[0]
		);
		std::process::exit(1);
	}

	let input_file: String = fs::read_to_string(&args[1])?;
	let target_dir: PathBuf = PathBuf::from(&args[2]);
	let input_json: Map<String, Value> = serde_json::from_str(&input_file)?;

	for (k, v) in input_json {
		let mut file_path = PathBuf::from(&target_dir);
		for part in k.split('.') {
			file_path.push(part);
		}
		file_path.set_extension("md");
		let file_dir: PathBuf = PathBuf::from(&file_path.parent().unwrap());
		fs::create_dir_all(file_dir)?;
		let mut file = OpenOptions::new()
			.create(true)
			.append(true)
			.open(file_path)?;
		writeln!(file, "- Name\n ```nix\n {k}\n ```")?;
		write_option(&mut file, v, 0)?;
	}
	return Ok(());
}

fn indent(level: u8) -> String {
	let mut str = String::new();
	for _ in 0..level {
		str.push('\t');
	}
	return str;
}

fn write_option(file: &mut File, value: Value, level: u8) -> io::Result<()> {
	match value {
		Value::Array(a) => {
			for v in a {
				write_option(file, v, level)?;
			}
		}
		Value::Bool(b) => {
			write!(file, " {}\n", b)?;
		}
		Value::Null => {
			write!(file, " NULL\n")?;
		}
		Value::Number(n) => {
			write!(file, " {}\n", n)?;
		}
		Value::Object(o) => {
			for (k, v) in o {
				write!(file, "{}- **{}:**", indent(level), k)?;
				match &v {
					Value::Array(_) => {
						write!(file, " [\n")?;
						write_option(file, v, level + 1)?;
						write!(file, "{}]\n", indent(level + 1))?;
					}
					Value::Bool(_) => {
						write!(file, " ")?;
						write_option(file, v, 0)?;
					}
					Value::Null => {
						write!(file, " ")?;
						write_option(file, v, 0)?;
					}
					Value::Number(_) => {
						write!(file, " ")?;
						write_option(file, v, 0)?;
					}
					Value::Object(_) => {
						write!(file, " {{\n")?;
						write_option(file, v, level + 1)?;
						write!(file, "{}}}\n", indent(level + 1))?;
					}
					Value::String(_) => {
						write!(file, " ")?;
						write_option(file, v, 0)?;
					}
				}
			}
		}
		Value::String(s) => {
			write!(file, " {}\n", s.replace("\n", " "))?;
		}
	}
	return Ok(());
}
