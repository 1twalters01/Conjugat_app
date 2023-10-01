mod args;

use clap::Parser;
use args::ScraperArgs;

fn main() {
    let args = ScraperArgs::parse();
    println!("{:?}", args);
}
