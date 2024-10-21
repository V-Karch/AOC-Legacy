fn main() {
    let contents = std::fs::read_to_string("input.txt")
        .expect("Unable to read file.");

    for line in contents.lines() {
        println!("{}", line);
    }
}
