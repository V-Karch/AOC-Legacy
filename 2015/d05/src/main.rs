fn main() {
    let contents = std::fs::read_to_string("input.txt")
        .expect("Unable to read file. Perhaps it's in the wrong location?");

    let part1_result = part1(&contents);
    println!("Part 1: {}", part1_result);
}

fn part1(contents: &String) -> u32 {
    let mut count: u32 = 0;

    for line in contents.split("\n") {
        if is_nice(line) {
            count += 1
        }
    }

    return count;
}

fn is_nice(line: &str) -> bool {
    return vowel_count(line) >= 3 && check_double_char(line) && !check_specific_strings(line);
}

fn check_specific_strings(line: &str) -> bool {
    return line.contains("ab")
        || line.contains("cd")
        || line.contains("pq")
        || line.contains("xy");
}

fn check_double_char(line: &str) -> bool {
    let mut previous: char = ' ';
    let mut previous_index: usize = 0;

    for (index, value) in line.chars().enumerate() {
        if index == 0 {
            previous = value;
            previous_index = 0;
            continue;
        }

        if value == previous && index == previous_index + 1 {
            return true;
        } else {
            previous = value;
            previous_index = index;
        }
    }

    return false;
}

fn vowel_count(line: &str) -> u8 {
    let mut vowel_count: u8 = 0;

    for i in line.chars() {
        match i {
            'a' => vowel_count += 1,
            'e' => vowel_count += 1,
            'i' => vowel_count += 1,
            'o' => vowel_count += 1,
            'u' => vowel_count += 1,
            _ => (),
        }
    }

    return vowel_count;
}
