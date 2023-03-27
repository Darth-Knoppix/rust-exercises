fn main() {
    let query = "picture";
    let quote = "\
    Every face, every shop, bedroom windor, public-house, and
    dark square is a picture feverishly turned--in search of what?
    It is the same with books.
    What do we seek through millions of pages?
    ";

    for line in quote.lines() {
        if line.contains(query) {
            println!("{}", line)
        }
    }
}
