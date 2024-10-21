fn main() {
    let contents = std::fs::read_to_string("input.txt")
        .expect("Unable to read file. Perhaps it is in the wrong location?");

    println!("{}", contents);
}
