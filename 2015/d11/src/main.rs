fn main() {
    let mut password: String = std::fs::read_to_string("input.txt").expect("failed to read input.txt");
    let part1_result: String = find_next_valid_password(&password);
    println!("Part 1: {}", part1_result);

    password = increment_password(&part1_result);
    let part2_result: String = find_next_valid_password(&password);
    println!("Part 2: {}", part2_result);
}

fn find_next_valid_password(password: &str) -> String {
    let mut password = increment_password(password);
    while !is_valid_password(&password) {
        password = increment_password(&password);
    }
    password
}

fn increment_password(password: &str) -> String {
    let mut chars: Vec<char> = password.chars().collect();
    let mut index = chars.len() - 1;

    loop {
        if chars[index] == 'z' {
            chars[index] = 'a';
            if index == 0 {
                break;
            }
            index -= 1;
        } else {
            chars[index] = (chars[index] as u8 + 1) as char;
            break;
        }
    }
    return chars.into_iter().collect();
}

fn is_valid_password(password: &str) -> bool {
    return has_straight(password) && !contains_invalid_chars(password) && has_two_pairs(password);
}

fn has_straight(password: &str) -> bool {
    let chars: Vec<char> = password.chars().collect();
    for i in 0..(chars.len() - 2) {
        if chars[i] as u8 + 1 == chars[i + 1] as u8 && chars[i] as u8 + 2 == chars[i + 2] as u8 {
            return true;
        }
    }
    return false;
}

fn contains_invalid_chars(password: &str) -> bool {
    return password.contains('i') || password.contains('o') || password.contains('l');
}

fn has_two_pairs(password: &str) -> bool {
    let mut count = 0;
    let chars: Vec<char> = password.chars().collect();
    let mut i = 0;

    while i < chars.len() - 1 {
        if chars[i] == chars[i + 1] {
            count += 1;
            i += 2; // Skip the next character to ensure non-overlapping pairs.
        } else {
            i += 1;
        }
    }

    return count >= 2;
}
