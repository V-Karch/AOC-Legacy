fn main() {
    let contents: String = std::fs::read_to_string("input.txt")
        .expect("Unable to read `input.txt`. Is it in the correct location?");

    println!("{}", contents);
}
