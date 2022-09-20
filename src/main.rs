mod adapter;
mod config;
mod runner;
mod selector_match;
mod test_file;
mod test_index;
mod test_selector;
mod utils;

#[cfg(test)]
mod test_env;

use std::error::Error;

use crate::test_selector::TestSelector;
use clap::Parser;

/// A CLI tool for running tests for any programming language.
///
/// Taking inspiration from [`vim-test`](https://github.com/vim-test/vim-test),
/// this tool makes it running test suites fun again.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    test_selectors: Vec<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let selectors = parse_selectors(&args)?;

    runner::run_all(&selectors)?;

    Ok(())
}

fn parse_selectors(args: &Args) -> Result<Vec<TestSelector>, Box<dyn Error>> {
    let mut selectors = vec![];

    for selector in &args.test_selectors {
        let selector = selector.parse::<TestSelector>()?;
        selectors.push(selector);
    }

    Ok(selectors)
}
