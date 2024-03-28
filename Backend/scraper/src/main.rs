#[warn(dead_code)]
mod args;
mod data_types;

mod functionality;
mod helper_functions;
mod main_processes;

use args::{ScrapeMode, ScraperArgs};
use clap::Parser;
use main_processes::{continue_process, initialise_process};

use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let args = ScraperArgs::parse();

    match args.scrape_mode {
        ScrapeMode::Initialise => {
            initialise_process().await;
        }
        ScrapeMode::Continue => {
            continue_process();
        }
    };
}

#[cfg(test)]
mod tests;
