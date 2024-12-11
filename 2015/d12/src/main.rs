use serde_json::Value;

fn main() {
    let contents = std::fs::read_to_string("input.json").expect("Failed to read input file");
    let part1_result = part1(&contents);
    let part2_result = part2(&contents);

    println!("Part 1: {}", part1_result);
    println!("Part 2: {}", part2_result);
}

fn part2(contents: &String) -> i32 {
    let as_json: Value = serde_json::from_str(contents)
        .expect("Failed to read input into json");

    let numeric_value = recurse(&as_json);
    return numeric_value;
}

fn recurse(value: &Value) -> i32 {
    return match value {
        Value::Number(num) => num.as_i64().unwrap_or(0) as i32,
        Value::Array(arr) => arr.iter().map(recurse).sum(),
        Value::Object(map) => {
            if map.values().any(|v| v == "red") {
                0
            } else {
                map.values().map(recurse).sum()
            }
        }
        _ => 0,
    };
}


fn part1(contents: &String) -> i32 {
    let mut in_number = false;
    let mut temp_num: Vec<char> = Vec::new();
    let mut sum = 0;

    for c in contents.chars() {
        if is_number_part(c) {
            in_number = true;
        }

        if in_number && is_number_part(c) {
            temp_num.push(c);
        }

        if in_number && is_number_part(c) == false {
            in_number = false;

            let value = temp_num
                .iter()
                .collect::<String>()
                .parse::<i32>()
                .expect("Failed to parse Vec");

            sum += value;
            temp_num.clear();
        }
    }

    return sum;
}

fn is_number_part(c: char) -> bool {
    return match c {
        '-' => true,
        '0' => true,
        '1' => true,
        '2' => true,
        '3' => true,
        '4' => true,
        '5' => true,
        '6' => true,
        '7' => true,
        '8' => true,
        '9' => true,
        _ => false,
    };
}
