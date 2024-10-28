#[derive(Debug)]
struct Edge {
    base: String,
    goal: String,
    distance: u32
}

fn main() {
    let contents = std::fs::read_to_string("input.txt")
        .expect("Unable to read file.");

    for line in contents.lines() {
        let pieces: Vec<&str> = line.split(" ").collect();

        let base: &str = pieces[0];
        let goal: &str = pieces[2];
        let distance: u32 = pieces[4].parse::<u32>().expect("Failed to parse int");

        let edge = Edge {
            base: (base).to_string(),
            goal: (goal).to_string(),
            distance
        };
        
        println!("{:?}", edge);
    }
}
