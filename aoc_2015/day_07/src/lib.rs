use regex::Regex;
use std::collections::BTreeMap;

pub fn part_one(input: &str) -> u16 {
    let circuit = init(input.trim());
    signal("a", &circuit, &mut BTreeMap::new())
}

pub fn part_two(input: &str) -> u16 {
    let circuit = init(input.trim());
    let mut signals = BTreeMap::new();
    let a = signal("a", &circuit, &mut signals);
    signals.clear();
    signals.insert("b", a);
    signal("a", &circuit, &mut signals)
}

fn init(instructions: &str) -> BTreeMap<&str, Gate> {
    let mut map = BTreeMap::new();

    let buffer_regex = Regex::new(r"^([[:alnum:]]+) -> ([[:alpha:]]+)$").unwrap();
    let binary_gate_regex =
        Regex::new(r"^([[:alnum:]]+) (AND|OR) ([[:alpha:]]+) -> ([[:alpha:]]+)$").unwrap();
    let shift_regex =
        Regex::new(r"^([[:alpha:]]+) (LSHIFT|RSHIFT) (\d+) -> ([[:alpha:]]+)$").unwrap();
    let unary_gate_regex = Regex::new(r"^(NOT) ([[:alpha:]]+) -> ([[:alpha:]]+)$").unwrap();

    for instruction in instructions.lines() {
        let (gate, wire) = if let Some(caps) = buffer_regex.captures(instruction) {
            let input = caps.get(1).unwrap().as_str();
            let input = match input.parse::<u16>() {
                Ok(x) => Signal::Value(x),
                Err(_) => Signal::Wire(input),
            };
            let gate = Gate::Buffer { input };
            let wire = caps.get(2).unwrap().as_str();
            (gate, wire)
        } else if let Some(caps) = binary_gate_regex.captures(instruction) {
            let gate = match caps.get(2).unwrap().as_str() {
                "AND" => {
                    let left = caps.get(1).unwrap().as_str();
                    let left = match left.parse::<u16>() {
                        Ok(x) => Signal::Value(x),
                        Err(_) => Signal::Wire(left),
                    };
                    Gate::And {
                        left,
                        right: caps.get(3).unwrap().as_str(),
                    }
                }
                "OR" => Gate::Or {
                    left: caps.get(1).unwrap().as_str(),
                    right: caps.get(3).unwrap().as_str(),
                },
                _ => unreachable!("Binary gate matched: '{}'", instruction),
            };
            let wire = caps.get(4).unwrap().as_str();
            (gate, wire)
        } else if let Some(caps) = shift_regex.captures(instruction) {
            let gate = match caps.get(2).unwrap().as_str() {
                "LSHIFT" => Gate::LeftShift {
                    left: caps.get(1).unwrap().as_str(),
                    right: caps.get(3).unwrap().as_str().parse().unwrap(),
                },
                "RSHIFT" => Gate::RightShift {
                    left: caps.get(1).unwrap().as_str(),
                    right: caps.get(3).unwrap().as_str().parse().unwrap(),
                },
                _ => unreachable!("Shift gate matched: '{}'", instruction),
            };
            let wire = caps.get(4).unwrap().as_str();
            (gate, wire)
        } else if let Some(caps) = unary_gate_regex.captures(instruction) {
            let gate = match caps.get(1).unwrap().as_str() {
                "NOT" => Gate::Not {
                    input: caps.get(2).unwrap().as_str(),
                },
                _ => unreachable!("Unary gate matched: '{}'", instruction),
            };
            let wire = caps.get(3).unwrap().as_str();
            (gate, wire)
        } else {
            panic!("invalid instruction: {}", instruction)
        };
        map.insert(wire, gate);
    }
    map
}

fn signal<'a>(
    wire: &'a str,
    circuit: &BTreeMap<&'a str, Gate<'a>>,
    signals: &mut BTreeMap<&'a str, u16>,
) -> u16 {
    if let Some(x) = signals.get(wire) {
        return *x;
    }
    let gate = circuit.get(wire).expect("unknown wire");
    let signal = match gate {
        Gate::Buffer { input, .. } => match input {
            Signal::Value(x) => *x,
            Signal::Wire(input) => match signals.get(input) {
                Some(x) => *x,
                None => signal(input, circuit, signals),
            },
        },
        Gate::And { left, right, .. } => {
            let left_signal = match left {
                Signal::Value(x) => *x,
                Signal::Wire(left) => match signals.get(left) {
                    Some(x) => *x,
                    None => {
                        let signal = signal(left, circuit, signals);
                        signals.insert(left, signal);
                        signal
                    }
                },
            };
            let right_signal = match signals.get(right) {
                Some(x) => *x,
                None => {
                    let right_signal = signal(right, circuit, signals);
                    signals.insert(right, right_signal);
                    right_signal
                }
            };
            left_signal & right_signal
        }
        Gate::Or { left, right, .. } => {
            let left_signal = match signals.get(left) {
                Some(x) => *x,
                None => {
                    let left_signal = signal(left, circuit, signals);
                    signals.insert(left, left_signal);
                    left_signal
                }
            };
            let right_signal = match signals.get(right) {
                Some(x) => *x,
                None => {
                    let right_signal = signal(right, circuit, signals);
                    signals.insert(right, right_signal);
                    right_signal
                }
            };
            left_signal | right_signal
        }
        Gate::RightShift { left, right, .. } => {
            let left_signal = match signals.get(left) {
                Some(x) => *x,
                None => {
                    let left_signal = signal(left, circuit, signals);
                    signals.insert(left, left_signal);
                    left_signal
                }
            };
            left_signal >> right
        }
        Gate::LeftShift { left, right, .. } => {
            let left_signal = match signals.get(left) {
                Some(x) => *x,
                None => {
                    let left_signal = signal(left, circuit, signals);
                    signals.insert(left, left_signal);
                    left_signal
                }
            };
            left_signal << right
        }
        Gate::Not { input, .. } => {
            let input_signal = match signals.get(input) {
                Some(x) => *x,
                None => {
                    let input_signal = signal(input, circuit, signals);
                    signals.insert(input, input_signal);
                    input_signal
                }
            };
            !input_signal
        }
    };
    signals.insert(wire, signal);
    signal
}
enum Signal<'a> {
    Wire(&'a str),
    Value(u16),
}
enum Gate<'a> {
    Buffer { input: Signal<'a> },
    And { left: Signal<'a>, right: &'a str },
    Or { left: &'a str, right: &'a str },
    RightShift { left: &'a str, right: u16 },
    LeftShift { left: &'a str, right: u16 },
    Not { input: &'a str },
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
    let circuit = init(instructions.trim());
    let mut signals = BTreeMap::new();
    assert_eq!(72, signal("d", &circuit, &mut signals));
    assert_eq!(507, signal("e", &circuit, &mut signals));
    assert_eq!(492, signal("f", &circuit, &mut signals));
    assert_eq!(114, signal("g", &circuit, &mut signals));
    assert_eq!(65412, signal("h", &circuit, &mut signals));
    assert_eq!(65079, signal("i", &circuit, &mut signals));
    assert_eq!(123, signal("x", &circuit, &mut signals));
    assert_eq!(456, signal("y", &circuit, &mut signals));
}
