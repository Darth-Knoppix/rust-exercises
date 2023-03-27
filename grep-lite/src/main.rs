fn main() {
    let query = "picture";
    let quote = "\
    Every face, every shop, bedroom windor, public-house, and
    dark square is a picture feverishly turned--in search of what?
    It is the same with books.
    What do we seek through millions of pages?";

    for (index, line) in quote.lines().enumerate() {
        if line.contains(query) {
            println!("{}: {}", index + 1, line.trim())
        }
    }
}
