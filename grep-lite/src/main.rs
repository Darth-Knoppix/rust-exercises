use clap::Parser;
use regex::Regex;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Parser)]
#[command(name = "Grep Lite")]
#[command(author = "Seth C. <hello@sethcorker.com>")]
#[command(version = "0.1")]
#[command(about = "A terrible grep implementation in rust", long_about = None)]
struct Args {
    /// The pattern to search for
    pattern: String,

    /// a file to search
    #[arg(short, long)]
    file: String,
}

fn find_in_file(file: &File, regexp: &Regex) {
    let reader = BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        let extracted_line = line.unwrap();
        let contains_substring = regexp.find(&extracted_line);

        match contains_substring {
            Some(_) => println!("{}: {}", index, extracted_line.trim()),
            None => (),
        }
    }
}

fn print_file_error(error: &std::io::Error) {
    match error.kind() {
        std::io::ErrorKind::NotFound => println!("File not found!"),
        _ => println!("Error: {:?}", error.kind()),
    }
}

fn main() {
    let cli = Args::parse();

    let regexp = Regex::new(cli.pattern.as_str()).unwrap();
    let file_input = cli.file.as_str();

    match File::open(file_input) {
        Ok(file) => find_in_file(&file, &regexp),
        Err(err) => print_file_error(&err),
    }
}
