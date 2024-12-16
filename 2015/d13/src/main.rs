use std::collections::HashMap;

fn main() {
    let contents =
        std::fs::read_to_string("input.txt").expect("Failed to read puzzle input");

    let part1_result = part1(&contents);
    println!("Part 1: {}", part1_result);

    let part2_result = part2(&contents);
    println!("Part 2: {}", part2_result);
}

fn calculate_happiness(
    arrangement: &Vec<&String>,
    happiness_map: &HashMap<(String, String), i32>,
) -> i32 {
    let mut total_happiness = 0;
    let len = arrangement.len();

    for i in 0..len {
        let person = arrangement[i];
        let next_person = arrangement[(i + 1) % len];
        let prev_person = arrangement[(i + len - 1) % len];

        total_happiness += happiness_map
            .get(&(person.clone(), next_person.clone()))
            .unwrap_or(&0);
        total_happiness += happiness_map
            .get(&(person.clone(), prev_person.clone()))
            .unwrap_or(&0);
    }

    return total_happiness;
}

fn part1(contents: &str) -> i32 {
    let mut happiness_map: HashMap<(String, String), i32> = HashMap::new();
    let mut people: Vec<String> = vec![];

    for entry in contents.lines() {
        let split_input = entry.split(" ").collect::<Vec<&str>>();

        let from = split_input[0].to_string();
        let to = split_input[split_input.len() - 1]
            .trim_end_matches('.')
            .to_string();
        let value_sign = match split_input[2] {
            "gain" => 1,
            "lose" => -1,
            _ => 0,
        };

        let value = value_sign
            * split_input[3]
                .parse::<i32>()
                .expect("Failed to parse integer value");

        happiness_map.insert((from.clone(), to.clone()), value);

        if !people.contains(&from) {
            people.push(from);
        }
        if !people.contains(&to) {
            people.push(to);
        }
    }

    let mut max_happiness = i32::MIN;
    let mut permutations = vec![];

    fn generate_permutations<T>(arr: &mut Vec<T>, l: usize, result: &mut Vec<Vec<T>>)
    where
        T: Clone,
    {
        if l == arr.len() {
            result.push(arr.clone());
        } else {
            for i in l..arr.len() {
                arr.swap(l, i);
                generate_permutations(arr, l + 1, result);
                arr.swap(l, i);
            }
        }
    }

    let mut people_refs: Vec<&String> = people.iter().collect();
    generate_permutations(&mut people_refs, 0, &mut permutations);

    for arrangement in &permutations {
        let happiness = calculate_happiness(&arrangement, &happiness_map);
        if happiness > max_happiness {
            max_happiness = happiness;
        }
    }

    return max_happiness;
}

fn part2(contents: &str) -> i32 {
    let mut happiness_map: HashMap<(String, String), i32> = HashMap::new();
    let mut people: Vec<String> = vec![];

    for entry in contents.lines() {
        let split_input = entry.split(" ").collect::<Vec<&str>>();

        let from = split_input[0].to_string();
        let to = split_input[split_input.len() - 1]
            .trim_end_matches('.')
            .to_string();
        let value_sign = match split_input[2] {
            "gain" => 1,
            "lose" => -1,
            _ => 0,
        };

        let value = value_sign
            * split_input[3]
                .parse::<i32>()
                .expect("Failed to parse integer value");

        happiness_map.insert((from.clone(), to.clone()), value);

        if !people.contains(&from) {
            people.push(from);
        }
        if !people.contains(&to) {
            people.push(to);
        }
    }

    let your_name = "You".to_string();
    people.push(your_name.clone());

    for person in &people {
        happiness_map.insert((your_name.clone(), person.clone()), 0);
        happiness_map.insert((person.clone(), your_name.clone()), 0);
    }

    let mut max_happiness = i32::MIN;
    let mut permutations = vec![];

    fn generate_permutations<T>(arr: &mut Vec<T>, l: usize, result: &mut Vec<Vec<T>>)
    where
        T: Clone,
    {
        if l == arr.len() {
            result.push(arr.clone());
        } else {
            for i in l..arr.len() {
                arr.swap(l, i);
                generate_permutations(arr, l + 1, result);
                arr.swap(l, i);
            }
        }
    }

    let mut people_refs: Vec<&String> = people.iter().collect();
    generate_permutations(&mut people_refs, 0, &mut permutations);

    for arrangement in &permutations {
        let happiness = calculate_happiness(&arrangement, &happiness_map);
        if happiness > max_happiness {
            max_happiness = happiness;
        }
    }

    return max_happiness;
}
