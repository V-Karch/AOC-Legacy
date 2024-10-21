fn main() {
    let contents: String = std::fs::read_to_string("input.txt")
        .expect("File `input.txt` could not be read. Is it in the correct location?");

    let part1_result: i32 = part1(&contents);
    println!("Result of part 1: {}", part1_result);

    let part2_result: i32 = part2(&contents);
    println!("Result of part 2: {}", part2_result);
}

fn part1(contents: &String) -> i32 {
    let mut result: i32 = 0;

    for i in contents.chars() {
        if i == '(' {
            result += 1;
        } else {
            result -= 1;
        }
    }

    return result;
}

fn part2(contents: &String) -> i32 {
    let mut result: i32 = 0;

    for (index, value) in contents.chars().enumerate() {
        if value == '(' {
            result += 1;
        } else {
            result -= 1;
        }

        if result == -1 {
            return index as i32 + 1;
        }
    }

    return -1; // Not found
}
