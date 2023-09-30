use clap::Parser;

// A program to scrape Reverso
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The method used for creating the data pks
    #[clap(value_enum)]
    #[arg(short, long, default_value_t = )]
    method: Method,
}

#[derive(clap::ValueEnum, Clone, Debug)]
enum Method {
    Init,
    Cont,
}

fn main() {
    println!("Hello, world!");
}
