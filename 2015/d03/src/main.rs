fn main() {
    let contents = std::fs::read_to_string("input.txt")
        .expect("Unable to read file. Perhaps it is in the wrong location?");

    let part1_result = part1(&contents);
    println!("Part 1: {}", part1_result);
}

fn part1(contents: &String) -> usize {
    let mut visited: std::collections::HashSet<(i32, i32)> = std::collections::HashSet::new();
    let mut x = 0;
    let mut y = 0;

    for i in contents.chars() {
        match i {
            '^' => y += 1,
            'v' => y -= 1,
            '>' => x += 1,
            '<' => x -= 1,
            _ => (),
        }

        visited.insert((x, y));
    }

    return visited.len();
}