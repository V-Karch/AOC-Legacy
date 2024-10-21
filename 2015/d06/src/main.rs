fn main() {
    let contents = std::fs::read_to_string("input.txt")
        .expect("Unable to read file. Perhaps something is in the wrong location?");

    let part1_result = part1(&contents);
    println!("Part 1: {}", part1_result);

    let part2_result = part2(&contents);
    println!("Part 2: {}", part2_result);
}

fn part1(contents: &str) -> i32 {
    let mut grid: Vec<Vec<i32>> = vec![vec![0; 1000]; 1000];

    for line in contents.lines() {
        let line_as_args: Vec<&str> = line.split_whitespace().collect();
        let (sx, sy, ex, ey) = get_range_values(&line_as_args);

        if line_as_args[0] == "toggle" {
            set_grid(&mut grid, Operation::Toggle, sx, ex, sy, ey);
        } else if line_as_args[1] == "on" {
            set_grid(&mut grid, Operation::On, sx, ex, sy, ey);
        } else {
            set_grid(&mut grid, Operation::Off, sx, ex, sy, ey);
        }
    }

    return find_true(&grid);
}

fn part2(contents: &str) -> i32 {
    let mut grid: Vec<Vec<i32>> = vec![vec![0; 1000]; 1000];

    for line in contents.lines() {
        let line_as_args: Vec<&str> = line.split_whitespace().collect();
        let (sx, sy, ex, ey) = get_range_values(&line_as_args);

        if line_as_args[0] == "toggle" {
            set_grid_part2(&mut grid, Operation::Toggle, sx, ex, sy, ey);
        } else if line_as_args[1] == "on" {
            set_grid_part2(&mut grid, Operation::On, sx, ex, sy, ey);
        } else {
            set_grid_part2(&mut grid, Operation::Off, sx, ex, sy, ey);
        }
    }

    return calculate_total_brightness(&grid);
}

fn find_true(grid: &Vec<Vec<i32>>) -> i32 {
    let mut count = 0;

    for row in grid {
        for &light in row {
            if light == 1 {
                count += 1;
            }
        }
    }

    return count;
}

fn calculate_total_brightness(grid: &Vec<Vec<i32>>) -> i32 {
    return grid.iter().flat_map(|row| row.iter()).sum();
}

fn set_grid(grid: &mut Vec<Vec<i32>>, operation: Operation, sx: i32, ex: i32, sy: i32, ey: i32) {
    for x in sx..=ex {
        for y in sy..=ey {
            match operation {
                Operation::On => grid[x as usize][y as usize] = 1,
                Operation::Off => grid[x as usize][y as usize] = 0,
                Operation::Toggle => grid[x as usize][y as usize] ^= 1,
            }
        }
    }
}

fn set_grid_part2(
    grid: &mut Vec<Vec<i32>>,
    operation: Operation,
    sx: i32,
    ex: i32,
    sy: i32,
    ey: i32,
) {
    for x in sx..=ex {
        for y in sy..=ey {
            match operation {
                Operation::On => grid[x as usize][y as usize] += 1,
                Operation::Off => {
                    grid[x as usize][y as usize] = (grid[x as usize][y as usize] - 1).max(0);
                }
                Operation::Toggle => grid[x as usize][y as usize] += 2,
            }
        }
    }
}

enum Operation {
    On,
    Off,
    Toggle,
}

fn get_range_values(line_as_args: &Vec<&str>) -> (i32, i32, i32, i32) {
    let (sx, sy) = extract_starting_coordinates(line_as_args);
    let (ex, ey) = extract_ending_coordinates(line_as_args);

    return (sx, sy, ex, ey);
}

fn extract_starting_coordinates(line_as_args: &Vec<&str>) -> (i32, i32) {
    let start_values: &str = match line_as_args[0] {
        "toggle" => line_as_args[1],
        _ => line_as_args[2],
    };

    return parse_coordinates(start_values);
}

fn extract_ending_coordinates(line_as_args: &Vec<&str>) -> (i32, i32) {
    let end_values: &str = line_as_args[line_as_args.len() - 1];
    return parse_coordinates(end_values);
}

fn parse_coordinates(coordinate_str: &str) -> (i32, i32) {
    let values: Vec<i32> = coordinate_str
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    return (values[0], values[1]);
}
