use clap::Parser;
use regex::Regex;
use std::fs::File;
use std::io;
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

    /// The text to search
    input: Option<String>,

    /// a file to search
    #[arg(short, long)]
    file: Option<String>,
}

fn find_in_lines<T: BufRead + Sized>(reader: T, regexp: Regex) {
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

fn read_from_file(file_input: &str, regexp: Regex) {
    match File::open(file_input) {
        Ok(file) => {
            let reader = BufReader::new(file);
            find_in_lines(reader, regexp)
        }
        Err(err) => print_file_error(&err),
    }
}

fn read_from_stdin(regexp: Regex) {
    let stdin = io::stdin();
    let reader = stdin.lock();
    find_in_lines(reader, regexp);
}

fn main() {
    let cli = Args::parse();
    let regexp = Regex::new(cli.pattern.as_str()).unwrap();

    match cli.file {
        None => read_from_stdin(regexp),
        Some(file_input) => read_from_file(&file_input, regexp),
    }
}
