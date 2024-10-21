fn main() {
    let contents: String = std::fs::read_to_string("input.txt")
        .expect("File `input.txt` could not be read. Is it in the correct location?");

    println!("{}", contents);
}
