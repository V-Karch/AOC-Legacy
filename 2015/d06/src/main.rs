fn main() {
    let contents = std::fs::read_to_string("input.txt")
        .expect("Unable to read file. Perhaps something is in the wrong location?");

    let part1_result = part1(&contents);
    println!("Part 1: {}", part1_result);
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

    find_true(&grid)
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

    count
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

enum Operation {
    On,
    Off,
    Toggle,
}

fn get_range_values(line_as_args: &Vec<&str>) -> (i32, i32, i32, i32) {
    let (sx, sy) = if line_as_args[0] == "toggle" {
        let start_values: Vec<i32> = line_as_args[1]
            .split(',')
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        (start_values[0], start_values[1])
    } else {
        let start_values: Vec<i32> = line_as_args[2]
            .split(',')
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        (start_values[0], start_values[1])
    };

    let end_values: Vec<i32> = line_as_args[line_as_args.len() - 1]
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
    let (ex, ey) = (end_values[0], end_values[1]);

    (sx, sy, ex, ey)
}
