use std::io;

pub fn initialise_process() {
    println!("Enter the languages you would like to scrape below:");
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let trimmed_buffer: &str = buffer.trim();

    let languages: Vec<&str> = trimmed_buffer.split(", ").collect::<Vec<&str>>();
    println!("{:?}", languages);

    println!("initialise");
}


pub fn continue_process() {
    println!("Enter the languages you would like to scrape below:");
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let trimmed_buffer = buffer.trim();

    let languages = trimmed_buffer.split(", ").collect::<Vec<&str>>();
    println!("{:?}", languages);




    // TODO
    println!("What words would you like to scrape?");
    buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let trimmed_buffer = buffer.trim();

    // Split by lanugae
    let infinitives = trimmed_buffer.split(",").collect::<Vec<&str>>();
    println!("{:?}", infinitives);

    println!("continue");
}


