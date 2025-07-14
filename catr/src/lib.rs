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
    dbg!(config);
    Ok(())
}
