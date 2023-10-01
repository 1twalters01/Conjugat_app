use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct ScraperArgs {
    #[clap(subcommand)]
    pub scrape_mode: ScrapeMode,
}

#[derive(Debug, Subcommand)]
pub enum ScrapeMode {
    /// Create the first set of conjugations
    Initialise(InitCommand),
    
    /// Add additional conjugations
    Continue(ContCommand),
}

#[derive(Debug, Args)]
pub struct InitCommand {
    /// Languages that will be scraped
    #[arg(short, long, num_args = 1.., default_values_t = vec!["Spanish".to_string(), "Portuguese".to_string(), "Italian".to_string(), "French".to_string(), "English".to_string()], value_delimiter = ' ')]
    languages: Vec<String>,

    /// Run the group module
    #[arg(short, long, default_value_t = true)]
    group_module: bool,

    // Run the main module
    #[arg(short, long, default_value_t = true)]
    main_module: bool,
}

#[derive(Debug, Args)]
pub struct ContCommand {
    /// Language that will be scraped
    #[arg(short, long, default_value_t = String::from("Spanish"), value_delimiter = ' ')]
    language: String,

    /// Infinitives to be scraped
    #[arg(short, long, num_args = 1..)]
    infinitives: Vec<String>,
}
