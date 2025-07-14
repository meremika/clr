use std::{
    fs::File,
    io::{BufRead, BufReader, stdin},
};

use clap::Parser;

/// Rust cat
#[derive(Debug, Parser)]
#[command(version, author)]
pub struct Config {
    #[arg(default_value = "-", hide_default_value = true, value_name = "FILE")]
    files: Vec<String>,

    /// Number lines
    #[arg(short = 'n', long = "number", conflicts_with = "number_nonblank_lines")]
    number_lines: bool,

    /// Number nonblank lines
    #[arg(short = 'b', long = "number-nonblank")]
    number_nonblank_lines: bool,
}

pub fn run(config: Config) -> anyhow::Result<()> {
    for filename in config.files {
        match open(&filename) {
            Err(e) => eprintln!("Failed to open {}: {}", &filename, e),
            Ok(f) => {
                let mut n: u32 = 0;
                for line in f.lines() {
                    let line = line?;
                    let line_number = if config.number_lines
                        || (config.number_nonblank_lines && !line.is_empty())
                    {
                        n += 1;
                        &format!("{n:6}\t")
                    } else {
                        ""
                    };
                    println!("{line_number}{line}");
                }
            }
        }
    }
    Ok(())
}

fn open(filename: &str) -> anyhow::Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
