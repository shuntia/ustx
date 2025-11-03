use std::env;
use std::fs;
use std::io::{self, Read};
use std::process;

fn main() {
    if let Err(error) = run() {
        eprintln!("error: {error}");
        process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = env::args().skip(1);
    let input = if let Some(path) = args.next() {
        if path == "-" {
            read_stdin()?
        } else {
            fs::read_to_string(path)?
        }
    } else {
        read_stdin()?
    };

    let project = ustx::Project::from_yaml_str(&input)?;
    println!("{project:#?}");
    Ok(())
}

fn read_stdin() -> io::Result<String> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    Ok(buffer)
}
