use std::env;
use std::fs;
use std::io::{self, Read};
use std::process;
use std::str::FromStr;

fn main() {
    if let Err(error) = run() {
        eprintln!("error: {error}");
        process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = env::args().skip(1);
    let target_raw = args.next().ok_or_else(|| {
        "usage: cargo run --example convert <target-version> [path|-]".to_string()
    })?;
    let target = ustx::Version::from_str(&target_raw)?;

    let input = if let Some(path) = args.next() {
        if path == "-" {
            read_stdin()?
        } else {
            fs::read_to_string(path)?
        }
    } else {
        read_stdin()?
    };

    let mut project = ustx::Project::from_yaml_str(&input)?;
    project.convert_to(target)?;
    let output = project.to_yaml_string()?;
    print!("{output}");
    Ok(())
}

fn read_stdin() -> io::Result<String> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    Ok(buffer)
}
