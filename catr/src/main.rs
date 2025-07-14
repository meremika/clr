use catr::Config;
use clap::Parser;

fn main() -> anyhow::Result<()> {
    catr::run(Config::parse())
}
