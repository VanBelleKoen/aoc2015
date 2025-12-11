use crate::days::Solution;
use std::collections::HashMap;

pub struct Day07;

#[derive(Clone)]
enum Gate {
    Value(u16),
    Wire(String),
    And(String, String),
    Or(String, String),
    LShift(String, u16),
    RShift(String, u16),
    Not(String),
}

fn parse_input(input: &str) -> HashMap<String, Gate> {
    let mut circuit = HashMap::new();

    for line in input.lines() {
        let parts: Vec<&str> = line.split(" -> ").collect();
        let target = parts[1].to_string();
        let instruction = parts[0];

        let gate = if instruction.contains("AND") {
            let operands: Vec<&str> = instruction.split(" AND ").collect();
            Gate::And(operands[0].to_string(), operands[1].to_string())
        } else if instruction.contains("OR") {
            let operands: Vec<&str> = instruction.split(" OR ").collect();
            Gate::Or(operands[0].to_string(), operands[1].to_string())
        } else if instruction.contains("LSHIFT") {
            let operands: Vec<&str> = instruction.split(" LSHIFT ").collect();
            let shift = operands[1].parse().unwrap();
            Gate::LShift(operands[0].to_string(), shift)
        } else if instruction.contains("RSHIFT") {
            let operands: Vec<&str> = instruction.split(" RSHIFT ").collect();
            let shift = operands[1].parse().unwrap();
            Gate::RShift(operands[0].to_string(), shift)
        } else if instruction.starts_with("NOT") {
            let wire = instruction.strip_prefix("NOT ").unwrap();
            Gate::Not(wire.to_string())
        } else if let Ok(value) = instruction.parse::<u16>() {
            Gate::Value(value)
        } else {
            Gate::Wire(instruction.to_string())
        };

        circuit.insert(target, gate);
    }

    circuit
}

fn evaluate(wire: &str, circuit: &HashMap<String, Gate>, cache: &mut HashMap<String, u16>) -> u16 {
    // Check if it's a direct value
    if let Ok(value) = wire.parse::<u16>() {
        return value;
    }

    // Check cache
    if let Some(&value) = cache.get(wire) {
        return value;
    }

    // Evaluate the gate
    let gate = circuit.get(wire).unwrap().clone();
    let result = match gate {
        Gate::Value(v) => v,
        Gate::Wire(w) => evaluate(&w, circuit, cache),
        Gate::And(a, b) => evaluate(&a, circuit, cache) & evaluate(&b, circuit, cache),
        Gate::Or(a, b) => evaluate(&a, circuit, cache) | evaluate(&b, circuit, cache),
        Gate::LShift(w, shift) => evaluate(&w, circuit, cache) << shift,
        Gate::RShift(w, shift) => evaluate(&w, circuit, cache) >> shift,
        Gate::Not(w) => !evaluate(&w, circuit, cache),
    };

    cache.insert(wire.to_string(), result);
    result
}

impl Solution for Day07 {
    fn part1(&self, input: &str) -> String {
        let circuit = parse_input(input);
        let mut cache = HashMap::new();
        let result = evaluate("a", &circuit, &mut cache);
        result.to_string()
    }

    fn part2(&self, input: &str) -> String {
        // First, get the signal from wire 'a' in part 1
        let mut circuit = parse_input(input);
        let mut cache = HashMap::new();
        let a_signal = evaluate("a", &circuit, &mut cache);

        // Override wire 'b' with the signal from 'a'
        circuit.insert("b".to_string(), Gate::Value(a_signal));

        // Reset the cache and re-evaluate
        cache.clear();
        let result = evaluate("a", &circuit, &mut cache);
        result.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i
d -> a";
        let solution = Day07;
        assert_eq!(solution.part1(input), "72");
    }

    #[test]
    fn test_part2() {
        let input = "";
        let solution = Day07;
        assert_eq!(solution.part2(input), "expected");
    }
}
