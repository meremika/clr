use std::{
    fs::File,
    io::{BufRead, BufReader, Read, stdin},
};

use anyhow::Result;
use clap::Parser;

/// Rust head
#[derive(Debug, Parser)]
#[command(version)]
pub struct Config {
    /// Input file(s)
    #[arg(default_value = "-", hide_default_value = true, value_name = "FILE")]
    files: Vec<String>,

    /// Number of lines
    #[arg(long, short = 'n', default_value = "10", conflicts_with = "bytes")]
    lines: usize,

    /// Number of bytes
    #[arg(long, short = 'c')]
    bytes: Option<usize>,
}

pub fn run(config: Config) -> Result<()> {
    let num_files = config.files.len();
    for (file_num, filename) in config.files.iter().enumerate() {
        match open(filename) {
            Err(e) => eprintln!("{filename}: {e}"),
            Ok(mut f) => {
                if num_files > 1 {
                    println!(
                        "{}==> {filename} <==",
                        if file_num == 0 { "" } else { "\n" }
                    );
                }
                if let Some(n) = config.bytes {
                    let mut buf = vec![0; n];
                    let bytes_read = f.take(n as u64).read(&mut buf)?;
                    print!("{}", String::from_utf8_lossy(&buf[0..bytes_read]));
                } else {
                    let mut line = String::new();
                    for _ in 0..config.lines {
                        let n = f.read_line(&mut line)?;
                        if n == 0 {
                            break;
                        }
                        print!("{line}");
                        line.clear();
                    }
                }
            }
        }
    }
    Ok(())
}

fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
