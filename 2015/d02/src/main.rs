fn main() {
    let contents: String = std::fs::read_to_string("input.txt")
        .expect("Unable to read `input.txt`. Is it in the correct location?");

    let part1_result: u32 = part1(&contents);
    println!("Part 1: {}", part1_result);

    let part2_result: u32 = part2(&contents);
    println!("Part 2: {}", part2_result);
}

fn part1(contents: &String)  -> u32 {
    let mut total: u32 = 0;

    for line in contents.split("\n") {
        let split_line: Vec<u32> = line
            .split("x")
            .map(|s| s.parse::<u32>().unwrap())
            .collect();

        let l: u32 = split_line[0];
        let w: u32 = split_line[1];
        let h: u32 = split_line[2];

        total += get_surface_area_and_slack(l, w, h);
    }

    return total;
}

fn part2(contents: &String)  -> u32 {
    let mut total: u32 = 0;

    for line in contents.split("\n") {
        let split_line: Vec<u32> = line
            .split("x")
            .map(|s| s.parse::<u32>().unwrap())
            .collect();

        let l: u32 = split_line[0];
        let w: u32 = split_line[1];
        let h: u32 = split_line[2];

        total += 2*get_smallest_sum(l, w, h) + l*w*h;
    }

    return total;
}

fn get_surface_area_and_slack(l: u32, w: u32, h: u32) -> u32 {
    return 2*l*w + 2*w*h + 2*h*l + get_smallest_product(l, w, h);
}

fn get_smallest_product(l: u32, w: u32, h: u32) -> u32 {
    // Get the product of the 2 smallest provided values
    let mut values: Vec<u32> = vec![l, w, h];
    values.sort();

    return values[0] * values[1];
}

fn get_smallest_sum(l: u32, w: u32, h: u32) -> u32 {
    let mut values: Vec<u32> = vec![l, w, h];
    values.sort();

    return values[0] + values[1];
}