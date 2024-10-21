fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len()  != 2 {
        println!("Error: Incorrect usage");
        println!("Expected 1 argument but got {}", args.len() - 1);
        return;
    }

    let contents: String = std::fs::read_to_string(&args[1])
        .expect("File could not be read");

    println!("{}", contents);
}
