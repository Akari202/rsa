#![feature(future_join)]
extern crate core;

mod rsa;
mod math;
mod cli;
#[cfg(test)]
mod tests;

use std::error::Error;
use clap::builder::TypedValueParser;
use clap::Parser;
use crate::cli::Cli;

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    env_logger::Builder::new()
        .filter_level(cli.verbose.into())
        .init();
    cli.command.execute()
}
