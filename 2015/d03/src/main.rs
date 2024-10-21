fn main() {
    let contents = std::fs::read_to_string("input.txt")
        .expect("Unable to read file. Perhaps it is in the wrong location?");

    let part1_result: usize = part1(&contents);
    println!("Part 1: {}", part1_result);

    let part2_result: usize = part2(&contents);
    println!("Part 2: {}", part2_result);
}

fn part1(contents: &String) -> usize {
    let mut visited: std::collections::HashSet<(i32, i32)> = std::collections::HashSet::new();
    let mut x: i32 = 0;
    let mut y: i32 = 0;

    visited.insert((x, y));

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

fn part2(contents: &String) -> usize {
    let mut visited_houses: std::collections::HashSet<(i32, i32)> = std::collections::HashSet::new();
    
    let mut sx: i32 = 0;
    let mut sy: i32 = 0; 
    let mut rx: i32 = 0; 
    let mut ry: i32 = 0; 

    visited_houses.insert((sx, sy));

    for (i, direction) in contents.chars().enumerate() {
        if i % 2 == 0 { 
            match direction {
                '^' => sy += 1,
                'v' => sy -= 1,
                '>' => sx += 1,
                '<' => sx -= 1,
                _ => (),
            }

            visited_houses.insert((sx, sy));
        } else {
            match direction {
                '^' => ry += 1,
                'v' => ry -= 1,
                '>' => rx += 1,
                '<' => rx -= 1,
                _ => (),
            }

            visited_houses.insert((rx, ry));
        }
    }

    visited_houses.len()
}