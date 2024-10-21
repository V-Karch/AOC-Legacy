fn main() {
    let contents: String = std::fs::read_to_string("input.txt")
        .expect("File `input.txt` could not be read. Is it in the correct location?");

    let part1_result: i32 = part1(&contents);
    println!("Result of part 1: {}", part1_result);
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