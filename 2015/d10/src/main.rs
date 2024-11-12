fn look_and_say(input: &str, iterations: usize) -> String {
    let mut current_sequence: String = input.to_string();

    for _ in 0..iterations {
        let mut next_sequence: String = String::new();
        let mut chars = current_sequence.chars().peekable();

        while let Some(current_char) = chars.next() {
            let mut count = 1;

            // Count consecutive characters
            while let Some(&next_char) = chars.peek() {
                if next_char == current_char {
                    chars.next();
                    count += 1;
                } else {
                    break;
                }
            }

            // Append the count and the character to the new sequence
            next_sequence.push_str(&count.to_string());
            next_sequence.push(current_char);
        }

        current_sequence = next_sequence;
    }

    return current_sequence;
}

fn main() {
    let input: String = std::fs::read_to_string("input.txt").expect("failed to read input.txt");
    let iterations: usize = 40;
    let result: String = look_and_say(&input, iterations);

    println!(
        "The length of the result after {} iterations is: {}",
        iterations,
        result.len()
    );
}
