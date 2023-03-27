use regex::Regex;

fn main() {
    let regexp = Regex::new("picture").unwrap();
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
