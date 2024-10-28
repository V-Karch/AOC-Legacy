use std::collections::HashMap;

fn parse_instruction(line: &str) -> (&str, &str) {
    let parts: Vec<&str> = line.trim().split(" -> ").collect();
    (parts[0], parts[1])
}

fn evaluate_wire(
    wire: &str,
    wires: &HashMap<String, String>,
    evaluated: &mut HashMap<String, u16>,
) -> u16 {
    // If the wire is a number, return it as u16
    if let Ok(num) = wire.parse::<u16>() {
        return num;
    }

    // Return the cached result if already evaluated
    if let Some(&value) = evaluated.get(wire) {
        return value;
    }

    // Get the operation associated with the wire
    let operation = wires.get(wire).expect("Wire not found");

    let result = if operation.contains("AND") {
        let parts: Vec<&str> = operation.split(" AND ").collect();
        evaluate_wire(parts[0].trim(), wires, evaluated)
            & evaluate_wire(parts[1].trim(), wires, evaluated)
    } else if operation.contains("OR") {
        let parts: Vec<&str> = operation.split(" OR ").collect();
        evaluate_wire(parts[0].trim(), wires, evaluated)
            | evaluate_wire(parts[1].trim(), wires, evaluated)
    } else if operation.contains("LSHIFT") {
        let parts: Vec<&str> = operation.split(" LSHIFT ").collect();
        evaluate_wire(parts[0].trim(), wires, evaluated) << parts[1].trim().parse::<u16>().unwrap()
    } else if operation.contains("RSHIFT") {
        let parts: Vec<&str> = operation.split(" RSHIFT ").collect();
        evaluate_wire(parts[0].trim(), wires, evaluated) >> parts[1].trim().parse::<u16>().unwrap()
    } else if operation.starts_with("NOT") {
        let target = operation.split_whitespace().nth(1).unwrap();
        !evaluate_wire(target.trim(), wires, evaluated) & 0xFFFF
    } else {
        // Direct assignment case
        evaluate_wire(operation, wires, evaluated)
    };

    evaluated.insert(wire.to_string(), result & 0xFFFF); // Ensure result is within 16 bits
    result & 0xFFFF
}

fn override_wire_b_and_reset(
    a_signal: u16,
    wires: &mut HashMap<String, String>,
    evaluated: &mut HashMap<String, u16>,
) -> u16 {
    // Override wire b with the signal from wire a
    wires.insert("b".to_string(), a_signal.to_string());

    // Clear the evaluated cache
    evaluated.clear();

    // Evaluate wire a again to get the new signal
    evaluate_wire("a", wires, evaluated)
}

fn main() {
    let input = std::fs::read_to_string("input.txt").expect("Unable to read file");
    let mut wires = HashMap::new();

    for line in input.lines() {
        let (operation, target) = parse_instruction(line);
        wires.insert(target.to_string(), operation.to_string());
    }

    let mut evaluated = HashMap::new();
    let result_a = evaluate_wire("a", &wires, &mut evaluated);
    println!("Signal to wire a: {}", result_a);

    // Now, override wire b with the signal from wire a and reset others
    let new_signal_a = override_wire_b_and_reset(result_a, &mut wires, &mut evaluated);
    println!("New signal to wire a after overriding b: {}", new_signal_a);
}
