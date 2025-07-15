use anyhow::Result;
use clap::Parser;
use headr::Config;

fn main() -> Result<()> {
    headr::run(Config::parse())
}
