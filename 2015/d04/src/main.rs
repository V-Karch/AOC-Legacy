fn main() {
    let contents: String = std::fs::read_to_string("input.txt")
        .expect("Unable to read file. Perhaps something is in the wrong location?");

    let part1_result: String = part1(&contents);
    println!("Part 1: {}", part1_result);
}

fn part1(contents: &String) -> String {
    let mut number: i32 = 0;

    loop {
        // Combine the contents and the number to create the input string
        let contents_and_number: String = format!("{}{}", contents, number);
        let hash_string: String = format!("{:x}", md5::compute(&contents_and_number));

        if hash_string.starts_with("00000") {
            return format!("Number: {}, Hash: {}", number, hash_string);
        }

        number += 1;
    }
}