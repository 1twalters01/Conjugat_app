mod args;
mod main_processes;

use args::{ScraperArgs, ScrapeMode};
use clap::Parser;
use main_processes::{continue_process, initialise_process};

fn main() {
    let args = ScraperArgs::parse();

    match args.scrape_mode {
        ScrapeMode::Initialise => {
            initialise_process();
        },
        ScrapeMode::Continue => {
            continue_process();
        }
    }; 
}


