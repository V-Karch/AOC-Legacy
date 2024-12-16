#[derive(Debug)]
pub struct Edge {
    pub from: String,
    pub to: String,
    pub value: i32
}

fn main() {
    let contents = std::fs::read_to_string("sample_input.txt").expect("Failed to read puzzle input");

    let mut edges: Vec<Edge> = vec![]; 

    for entry in contents.lines() {
        let split_input = entry.split(" ").collect::<Vec<&str>>();

        let from = split_input[0];
        let to = split_input[split_input.len() - 1];
        let value_sign = match split_input[2] {
            "gain" => 1,
            "lose" => -1,
            _ => 0,
        };

        let value = value_sign * split_input[3]
            .parse::<i32>()
            .expect("Failed to parse integer value");

        let vertex = Edge {
            from: from.to_string(),
            to: to.to_string(),
            value,
        };

        edges.push(vertex);
    }

    println!("{:?}", edges);
}
