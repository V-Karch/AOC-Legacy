use itertools::Itertools;
use std::collections::HashMap;

fn main() {
    let contents: String = std::fs::read_to_string("input.txt").expect("Unable to read file.");

    let unique_places: Vec<&str> = Vec::from([
        "Tambi",
        "Snowdin",
        "Norrath",
        "Faerun",
        "Tristram",
        "Arbre",
        "Straylight",
        "AlphaCentauri",
    ]);

    let part1_result: u32 = part1(&contents, &unique_places);
    println!("Part 1: {}", part1_result);

    let part2_result: u32 = part2(&contents, &unique_places);
    println!("Part 2: {}", part2_result);
}

fn part1(contents: &String, unique_places: &Vec<&str>) -> u32 {
    let mut distance_map: HashMap<(&str, &str), u32> = HashMap::new();

    // Parse the input into edges
    for line in contents.lines() {
        let pieces: Vec<&str> = line.split(" ").collect();
        let base: &str = pieces[0];
        let goal: &str = pieces[2];
        let distance: u32 = pieces[4].parse::<u32>().expect("Failed to parse distance");

        // Add distance to the map (bidirectional)
        distance_map.insert((base, goal), distance);
        distance_map.insert((goal, base), distance); // Since the graph is undirected
    }

    // Initialize variables to track the shortest path
    let mut shortest_distance = u32::MAX;

    // Generate all permutations of the places
    for perm in unique_places.iter().permutations(unique_places.len()) {
        let mut current_distance = 0;
        let mut valid_path = true;

        // Calculate the total distance for this permutation
        for i in 0..perm.len() - 1 {
            let from = perm[i];
            let to = perm[i + 1];

            // Check if the edge exists
            if let Some(&dist) = distance_map.get(&(from, to)) {
                current_distance += dist;
            } else {
                valid_path = false;
                break;
            }
        }

        // If the path is valid and shorter than the current shortest, update
        if valid_path && current_distance < shortest_distance {
            shortest_distance = current_distance;
        }
    }

    return shortest_distance;
}

fn part2(contents: &String, unique_places: &Vec<&str>) -> u32 {
    let mut distance_map: HashMap<(&str, &str), u32> = HashMap::new();

    // Parse the input into edges (same as in part1)
    for line in contents.lines() {
        let pieces: Vec<&str> = line.split(" ").collect();
        let base: &str = pieces[0];
        let goal: &str = pieces[2];
        let distance: u32 = pieces[4].parse::<u32>().expect("Failed to parse distance");

        // Add distance to the map (bidirectional)
        distance_map.insert((base, goal), distance);
        distance_map.insert((goal, base), distance); // Since the graph is undirected
    }

    // Initialize variables to track the longest path
    let mut longest_distance = 0; // Start at 0 since we are looking for the longest

    // Generate all permutations of the places
    for perm in unique_places.iter().permutations(unique_places.len()) {
        let mut current_distance = 0;
        let mut valid_path = true;

        // Calculate the total distance for this permutation
        for i in 0..perm.len() - 1 {
            let from = perm[i];
            let to = perm[i + 1];

            // Check if the edge exists
            if let Some(&dist) = distance_map.get(&(from, to)) {
                current_distance += dist;
            } else {
                valid_path = false;
                break;
            }
        }

        // If the path is valid and longer than the current longest, update
        if valid_path && current_distance > longest_distance {
            longest_distance = current_distance;
        }
    }

    return longest_distance;
}
