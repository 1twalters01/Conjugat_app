use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct ScraperArgs {
    #[clap(subcommand)]
    pub scrape_mode: ScrapeMode,
}

#[derive(Debug, Subcommand)]
pub enum ScrapeMode {
    /// Create the first set of conjugations
    Initialise,
    
    /// Add additional conjugations
    Continue,
}

