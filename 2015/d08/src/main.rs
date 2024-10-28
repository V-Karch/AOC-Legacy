fn main() {
    let contents = std::fs::read_to_string("sample_input.txt")
        .expect("Failed to read file. Perhaps it was in the wrong location.");

    for line in contents.lines() {
        println!("{}", line);
    }
}
