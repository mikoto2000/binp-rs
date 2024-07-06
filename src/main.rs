use clap::Parser;

mod option_parser;

fn main() {
    let _ = option_parser::Options::parse();

    println!("Hello, world!");
}
