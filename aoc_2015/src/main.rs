use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

const FILE_TEXT: &str = include_str!("../input/day7.txt");

#[derive(PartialEq, Debug)]
enum Operation {
    NOOP,
    NOT,
    AND,
    OR,
    LSHIFT,
    RSHIFT,
    UNDEFINED,
}

impl Operation {
    fn can_run(&self, wire_a: Option<u16>, wire_b: Option<u16>) -> bool {
        match *self {
            Operation::NOOP | Operation::NOT => wire_a != None,
            _ => wire_a != None && wire_b != None,
        }
    }

    fn run(&self, wire_a: Option<u16>, wire_b: Option<u16>) -> Option<u16> {
        if self.can_run(wire_a, wire_b) {
            Some(match *self {
                Operation::NOOP => wire_a.unwrap(),
                Operation::NOT => !wire_a.unwrap(),
                Operation::AND => wire_a.unwrap() & wire_b.unwrap(),
                Operation::OR => wire_a.unwrap() | wire_b.unwrap(),
                Operation::LSHIFT => wire_a.unwrap() << wire_b.unwrap(),
                Operation::RSHIFT => wire_a.unwrap() >> wire_b.unwrap(),
                Operation::UNDEFINED => panic!(), // this value exists just for this
            })
        } else {
            None
        }
    }
}

fn eval_instruction(
    instruction: &str,
    wires: &HashMap<String, u16>,
) -> Option<(String, Option<u16>)> {
    let mut operation: Operation = Operation::UNDEFINED;
    let mut wire_a: Option<u16> = None;
    let mut wire_b: Option<u16> = None;
    let mut target: String = String::from("");

    lazy_static! {
        static ref RE_PATTERN_1: Regex =
            Regex::new(r"^\s*(|NOT?)\s*([a-z0-9]*)\s->\s([a-z]*)$").unwrap();
        static ref RE_PATTERN_2: Regex =
            Regex::new(r"^\s*([a-z0-9]*)\s(AND|OR|LSHIFT|RSHIFT)\s([a-z0-9]*)\s->\s([a-z]*)$")
                .unwrap();
    }

    if RE_PATTERN_1.is_match(instruction) {
        for cap in RE_PATTERN_1.captures_iter(instruction) {
            if &cap[1] == "NOT" {
                operation = Operation::NOT;
            } else {
                operation = Operation::NOOP;
            }
            wire_a = cap[2].parse::<u16>().ok();
            if wire_a == None {
                if let Some(wire_a_resolved) = wires.get(&cap[2]) {
                    wire_a = Some(*wire_a_resolved);
                }
            }
            wire_b = None;
            target = String::from(&cap[3]);
        }
    } else if RE_PATTERN_2.is_match(instruction) {
        for cap in RE_PATTERN_2.captures_iter(instruction) {
            operation = match &cap[2] {
                "AND" => Operation::AND,
                "OR" => Operation::OR,
                "LSHIFT" => Operation::LSHIFT,
                "RSHIFT" => Operation::RSHIFT,
                _ => Operation::UNDEFINED,
            };

            wire_a = cap[1].parse::<u16>().ok();
            if wire_a == None {
                if let Some(wire_a_resolved) = wires.get(&cap[1]) {
                    wire_a = Some(*wire_a_resolved);
                }
            }
            wire_b = cap[3].parse::<u16>().ok();
            if wire_b == None {
                if let Some(wire_b_resolved) = wires.get(&cap[3]) {
                    wire_b = Some(*wire_b_resolved);
                }
            }
            target = String::from(&cap[4]);
        }
    } else {
        panic!(); // Unrecoverable error. There is an error in the instructions.
    }
    Some((target, operation.run(wire_a, wire_b)))
}

fn wire_circuit(instructions: &str) -> HashMap<String, u16> {
    let mut wires: HashMap<String, u16> = HashMap::new();
    let mut pending_instructions = String::from("");
    let mut running_instructions = instructions.to_string();
    loop {
        for instruction in running_instructions.lines() {
            if let Some((key, output)) = eval_instruction(instruction, &wires) {
                if let Some(value) = output {
                    if let Some(value) = wires.get(&key) {
                        println!("Already exists {}", value);
                    }
                    wires.insert(key, value);
                } else {
                    pending_instructions = format!("{}\n{}", pending_instructions, &instruction);
                }
            } else {
                pending_instructions = format!("{}\n{}", pending_instructions, &instruction);
            }
        }
        if pending_instructions.is_empty() {
            break;
        } else {
            running_instructions = pending_instructions.trim().to_string();
            pending_instructions = String::from("");
        }
    }
    wires
}

fn main() {
    let wires = wire_circuit(FILE_TEXT);
    println!("{:?}", wires["a"]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wire_circuit() {
        let instructions = "123 -> x
                            456 -> y
                            x AND y -> d
                            x OR y -> e
                            x LSHIFT 2 -> f
                            y RSHIFT 2 -> g
                            NOT x -> h
                            NOT y -> i";
        let wires = wire_circuit(instructions);
        assert_eq!(wires["i"], 65079);
    }

    #[test]
    fn test_eval_instruction() {
        let wires = HashMap::new();
        assert_eq!(
            eval_instruction("123 -> x", &wires).unwrap(),
            (String::from("x"), Some(123))
        );
        assert_eq!(
            eval_instruction("NOT 0 -> x", &wires).unwrap(),
            (String::from("x"), Some(65535))
        );
        assert_eq!(
            eval_instruction("1 AND 1 -> x", &wires).unwrap(),
            (String::from("x"), Some(1))
        );
        assert_eq!(
            eval_instruction("0 AND 1 -> x", &wires).unwrap(),
            (String::from("x"), Some(0))
        );
        assert_eq!(
            eval_instruction("1 LSHIFT 2 -> x", &wires).unwrap(),
            (String::from("x"), Some(4))
        );
        assert_eq!(
            eval_instruction("1 RSHIFT 2 -> x", &wires).unwrap(),
            (String::from("x"), Some(0))
        );
    }
    #[test]
    fn test_operation_can_run() {
        assert!(Operation::NOOP.can_run(Some(0), None));
        assert!(!Operation::NOOP.can_run(None, Some(0)));
        assert!(Operation::NOT.can_run(Some(0), None));
        assert!(!Operation::NOT.can_run(None, Some(0)));
        assert!(Operation::AND.can_run(Some(0), Some(0)));
        assert!(!Operation::AND.can_run(None, Some(0)));
        assert!(Operation::OR.can_run(Some(0), Some(0)));
        assert!(!Operation::OR.can_run(Some(0), None));
        assert!(Operation::LSHIFT.can_run(Some(0), Some(0)));
        assert!(!Operation::LSHIFT.can_run(Some(0), None));
        assert!(Operation::RSHIFT.can_run(Some(0), Some(0)));
        assert!(!Operation::RSHIFT.can_run(Some(0), None));
    }
    #[test]
    fn test_operation_run() {
        assert_eq!(Operation::NOOP.run(Some(15), None), Some(15));
        assert_eq!(
            Operation::NOT.run(Some(0b0000000000000000), None),
            Some(0b1111111111111111)
        );
        assert_eq!(
            Operation::AND.run(Some(0b111111), Some(0b101010)),
            Some(0b101010)
        );
        assert_eq!(
            Operation::OR.run(Some(0b111111), Some(0b101010)),
            Some(0b111111)
        );
        assert_eq!(
            Operation::LSHIFT.run(Some(0b111111), Some(2)),
            Some(0b11111100)
        );
        assert_eq!(Operation::RSHIFT.run(Some(0b111111), Some(2)), Some(0b1111));
    }
}
