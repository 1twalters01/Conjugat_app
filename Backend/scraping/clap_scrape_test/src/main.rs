use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    cname: String,

    /// Number of times to greet
    // short not used as we would have an error as cname and cout would use -c
    #[arg(long, default_value_t = 1)]
    count: u8,

    /// Enum as an example
    #[clap(value_enum)]
    #[arg(default_value_t = Type::Info)]
    arg_type: Type,
}

#[derive(clap::ValueEnum, Clone, Debug)]
enum Type {
    None,
    Debug,
    Info,
    Test,
}

fn main() {
    let args = Args::parse();

    for _ in 0..args.count {
        println!("Hello {}!", args.cname);
        println!("{:?}", args.arg_type);
        println!("{:?}", args);
    }
}

