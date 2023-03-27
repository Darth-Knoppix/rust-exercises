fn main() {
    let ctx_lines = 1;
    let query = "picture";
    let quote = "\
    Every face, every shop, bedroom windor, public-house, and
    dark square is a picture feverishly turned--in search of what?
    It is the same with books.
    What do we seek through millions of pages?";

    let mut tags: Vec<usize> = vec![];
    let mut ctx: Vec<Vec<(usize, String)>> = vec![];

    for (index, line) in quote.lines().enumerate() {
        if line.contains(query) {
            tags.push(index);
            let v = Vec::with_capacity(2 * ctx_lines + 1);
            ctx.push(v)
        }
    }

    if tags.is_empty() {
        return;
    }

    for (line_index, line) in quote.lines().enumerate() {
        for (tag_index, tag) in tags.iter().enumerate() {
            let lower_bound = tag.saturating_sub(ctx_lines);
            let upper_bound = tag + ctx_lines;

            if (line_index >= lower_bound) && (line_index <= upper_bound) {
                let line_as_string = String::from(line);
                let local_ctx = (line_index, line_as_string);
                ctx[tag_index].push(local_ctx);
            }
        }
    }

    for local_ctx in ctx.iter() {
        for &(index, ref line) in local_ctx.iter() {
            let line_number = index + 1;
            println!("{}: {}", line_number, line.trim());
        }
    }
}
