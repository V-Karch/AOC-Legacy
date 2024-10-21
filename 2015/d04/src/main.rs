fn main() {
    let contents = std::fs::read_to_string("input.txt")
        .expect("");

    println!("{}", contents);
}
