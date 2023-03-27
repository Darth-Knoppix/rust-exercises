use clap::Parser;
use regex::Regex;

#[derive(Parser)]
#[command(name = "Grep Lite")]
#[command(author = "Seth C. <hello@sethcorker.com>")]
#[command(version = "0.1")]
#[command(about = "A terrible grep implementation in rust", long_about = None)]
struct Args {
    /// The pattern to search for
    pattern: String,
}

fn main() {
    let cli = Args::parse();

    let regexp = Regex::new(cli.pattern.as_str()).unwrap();
    let quote = "\
    Every face, every shop, bedroom windor, public-house, and
    dark square is a picture feverishly turned--in search of what?
    It is the same with books.
    What do we seek through millions of pages?";

    for (index, line) in quote.lines().enumerate() {
        let contains_substring = regexp.find(line);

        match contains_substring {
            Some(_) => println!("{}: {}", index, line.trim()),
            None => (),
        }
    }
}
