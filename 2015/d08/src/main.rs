fn main() {
    let contents: String =
        std::fs::read_to_string("input.txt").expect("Unable to read file.");

    let part1_result: usize = part1(&contents);
    println!("Part 1: {}", part1_result);
}

fn part1(contents: &String) -> usize {
    let mut total_code_characters: usize = 0;
    let mut total_memory_characters: usize = 0;

    for line in contents.lines() {
        let code_length: usize = line.len();
        let memory_length: usize = eval_string_memory_length(line);

        total_code_characters += code_length;
        total_memory_characters += memory_length;
    }

    let difference: usize = total_code_characters - total_memory_characters;

    return difference;
}

fn eval_string_memory_length(s: &str) -> usize {
    let mut memory_length: usize = 0;
    let mut chars: std::iter::Peekable<std::str::Chars<'_>> = s.chars().peekable();

    // Skip the first and last character as they are the enclosing quotes
    chars.next();
    chars.next_back();

    // Iterate through the string using a loop
    while let Some(c) = chars.next() {
        match c {
            '\\' => {
                // Peek ahead to check the next character
                if let Some(&next_char) = chars.peek() {
                    match next_char {
                        '\\' | '"' => {
                            memory_length += 1;
                            chars.next(); // Consume the next character
                        }
                        'x' => {
                            // Skip the 'x' and the two hexadecimal characters
                            chars.next(); // Consume 'x'
                            chars.next(); // Skip first hex digit
                            chars.next(); // Skip second hex digit
                            memory_length += 1;
                        }
                        _ => {}
                    }
                }
            }
            _ => memory_length += 1, // Regular character
        }
    }

    return memory_length;
}
