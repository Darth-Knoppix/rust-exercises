fn main() {
    greet_world()
}

fn greet_world() {
    let eng = "Hello, Wrold!";
    let jp = "ハロー・ワールド";
    let gr = "Grüß Gott!";
    let emj = "👋🌎";
    for locale in [eng, jp, gr, emj].iter() {
        println!("{}", &locale)
    }
}
