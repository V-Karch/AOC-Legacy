fn main() {
    let contents = std::fs::read_to_string("input.txt")
        .expect("Unable to read file. Perhaps it's in the wrong location?");

    println!("{}", &contents);
}
