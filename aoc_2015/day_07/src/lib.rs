use regex::Regex;
use std::collections::BTreeMap;

pub fn part_one(input: &str) -> u16 {
    let gates = init_gates(input.trim());
    let mut wires = init_wires(&gates);
    emulate_circuit(&gates, &mut wires);
    wires.get("a").expect("wire 'a' not found").unwrap()
}

pub fn part_two(input: &str) -> u16 {
    let gates = init_gates(input.trim());
    let mut wires = init_wires(&gates);
    emulate_circuit(&gates, &mut wires);
    let a = wires.get("a").expect("wire 'a' nod found").unwrap();
    for value in wires.values_mut() {
        *value = None
    }
    wires.insert("b", Some(a));
    emulate_circuit(&gates, &mut wires);
    wires.get("a").expect("wire 'a' not found").unwrap()
}

fn emulate_circuit<'a>(gates: &[Gate<'a>], wires: &mut BTreeMap<&'a str, Option<u16>>) {
    while wires.values().any(|o| o.is_none()) {
        emulate(&gates, wires);
    }
}

fn init_gates(instructions: &str) -> Vec<Gate> {
    let buffer_regex = Regex::new(r"^([[:alnum:]]+) -> ([[:alpha:]]+)$").unwrap();
    let binary_gate_regex =
        Regex::new(r"^([[:alnum:]]+) (AND|OR) ([[:alpha:]]+) -> ([[:alpha:]]+)$").unwrap();
    let shift_regex =
        Regex::new(r"^([[:alpha:]]+) (LSHIFT|RSHIFT) (\d+) -> ([[:alpha:]]+)$").unwrap();
    let unary_gate_regex = Regex::new(r"^(NOT) ([[:alpha:]]+) -> ([[:alpha:]]+)$").unwrap();

    let mut gates = Vec::new();
    for instruction in instructions.lines() {
        let gate = if let Some(caps) = buffer_regex.captures(instruction) {
            let input = caps.get(1).unwrap().as_str();
            let input = match input.parse::<u16>() {
                Ok(x) => Signal::Value(x),
                Err(_) => Signal::Wire(input),
            };
            Gate::Buffer {
                input,
                output: caps.get(2).unwrap().as_str(),
            }
        } else if let Some(caps) = binary_gate_regex.captures(instruction) {
            match caps.get(2).unwrap().as_str() {
                "AND" => {
                    let left = caps.get(1).unwrap().as_str();
                    let left = match left.parse::<u16>() {
                        Ok(x) => Signal::Value(x),
                        Err(_) => Signal::Wire(left),
                    };
                    Gate::And {
                        left,
                        right: caps.get(3).unwrap().as_str(),
                        output: caps.get(4).unwrap().as_str(),
                    }
                }
                "OR" => Gate::Or {
                    left: caps.get(1).unwrap().as_str(),
                    right: caps.get(3).unwrap().as_str(),
                    output: caps.get(4).unwrap().as_str(),
                },
                _ => unreachable!("Binary gate matched: '{}'", instruction),
            }
        } else if let Some(caps) = shift_regex.captures(instruction) {
            match caps.get(2).unwrap().as_str() {
                "LSHIFT" => Gate::LeftShift {
                    left: caps.get(1).unwrap().as_str(),
                    right: caps.get(3).unwrap().as_str().parse().unwrap(),
                    output: caps.get(4).unwrap().as_str(),
                },
                "RSHIFT" => Gate::RightShift {
                    left: caps.get(1).unwrap().as_str(),
                    right: caps.get(3).unwrap().as_str().parse().unwrap(),
                    output: caps.get(4).unwrap().as_str(),
                },
                _ => unreachable!("Shift gate matched: '{}'", instruction),
            }
        } else if let Some(caps) = unary_gate_regex.captures(instruction) {
            match caps.get(1).unwrap().as_str() {
                "NOT" => Gate::Not {
                    input: caps.get(2).unwrap().as_str(),
                    output: caps.get(3).unwrap().as_str(),
                },
                _ => unreachable!("Unary gate matched: '{}'", instruction),
            }
        } else {
            panic!("invalid instruction: {}", instruction)
        };
        gates.push(gate);
    }
    gates
}

fn init_wires<'a>(gates: &[Gate<'a>]) -> BTreeMap<&'a str, Option<u16>> {
    let mut map = BTreeMap::new();
    for gate in gates {
        map.insert(gate.output(), None);
    }
    map
}

fn emulate<'a>(gates: &[Gate<'a>], wires: &mut BTreeMap<&'a str, Option<u16>>) {
    for gate in gates {
        let output = gate.output();
        if wires.get_mut(output).unwrap().is_none() {
            let signal = gate.emulate(wires);
            wires.insert(output, signal);
        }
    }
}
enum Signal<'a> {
    Wire(&'a str),
    Value(u16),
}
enum Gate<'a> {
    Buffer {
        input: Signal<'a>,
        output: &'a str,
    },
    And {
        left: Signal<'a>,
        right: &'a str,
        output: &'a str,
    },
    Or {
        left: &'a str,
        right: &'a str,
        output: &'a str,
    },
    RightShift {
        left: &'a str,
        right: u16,
        output: &'a str,
    },
    LeftShift {
        left: &'a str,
        right: u16,
        output: &'a str,
    },
    Not {
        input: &'a str,
        output: &'a str,
    },
}

impl<'a> Gate<'a> {
    fn output(&self) -> &'a str {
        match self {
            Gate::Buffer { output, .. } => output,
            Gate::And { output, .. } => output,
            Gate::Or { output, .. } => output,
            Gate::RightShift { output, .. } => output,
            Gate::LeftShift { output, .. } => output,
            Gate::Not { output, .. } => output,
        }
    }

    fn emulate(&self, wires: &BTreeMap<&'a str, Option<u16>>) -> Option<u16> {
        match self {
            Gate::Buffer { input, .. } => match input {
                Signal::Value(x) => Some(*x),
                Signal::Wire(wire) => match wires.get(wire).unwrap() {
                    Some(x) => Some(*x),
                    None => None,
                },
            },
            Gate::And { left, right, .. } => {
                let left = match left {
                    Signal::Value(x) => *x,
                    Signal::Wire(wire) => match wires.get(wire).unwrap() {
                        Some(x) => *x,
                        None => return None,
                    },
                };
                match wires.get(right).unwrap() {
                    Some(right) => Some(left & *right),
                    None => None,
                }
            }
            Gate::Or { left, right, .. } => {
                match wires.get(left).unwrap().zip(*wires.get(right).unwrap()) {
                    Some((left, right)) => Some(left | right),
                    None => None,
                }
            }
            Gate::RightShift { left, right, .. } => match wires.get(left).unwrap() {
                Some(left) => Some(*left >> *right),
                None => None,
            },
            Gate::LeftShift { left, right, .. } => match wires.get(left).unwrap() {
                Some(left) => Some(*left << *right),
                None => None,
            },
            Gate::Not { input, .. } => match wires.get(input).unwrap() {
                Some(input) => Some(!input),
                None => None,
            },
        }
    }
}

#[test]
fn part_one_example() {
    let instructions = "\
    123 -> x\n\
    456 -> y\n\
    x AND y -> d\n\
    x OR y -> e\n\
    x LSHIFT 2 -> f\n\
    y RSHIFT 2 -> g\n\
    NOT x -> h\n\
    NOT y -> i\n";
    let gates = init_gates(instructions);
    let mut wires = init_wires(&gates);
    emulate_circuit(&gates, &mut wires);
    assert_eq!(Some(72), *wires.get("d").unwrap());
    assert_eq!(Some(507), *wires.get("e").unwrap());
    assert_eq!(Some(492), *wires.get("f").unwrap());
    assert_eq!(Some(114), *wires.get("g").unwrap());
    assert_eq!(Some(65412), *wires.get("h").unwrap());
    assert_eq!(Some(65079), *wires.get("i").unwrap());
    assert_eq!(Some(123), *wires.get("x").unwrap());
    assert_eq!(Some(456), *wires.get("y").unwrap());
}

#[test]
fn part_two_example() {}
