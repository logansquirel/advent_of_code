pub fn puzzle_one(input: &str) -> i32 {
    let mut vec: Vec<i32> = input
        .trim()
        .split(',')
        .filter_map(|x| x.parse::<i32>().ok())
        .collect();
    vec[1] = 12;
    vec[2] = 2;
    IntCode::execute(&mut vec);
    vec[0]
}

pub fn puzzle_two(input: &str) -> i32 {
    let initial_program: Vec<i32> = input
        .trim()
        .split(',')
        .filter_map(|x| x.parse::<i32>().ok())
        .collect();
    for noun in 0..100 {
        for verb in 0..100 {
            let mut program = initial_program.clone();
            program[1] = noun;
            program[2] = verb;
            IntCode::execute(&mut program);
            if program[0] == 19690720 {
                return 100 * noun + verb;
            }
        }
    }
    panic!("no gravity assist solution found")
}

enum OpCode {
    Add,
    Multiply,
}

impl OpCode {
    fn from(opcode: i32) -> Option<Self> {
        match opcode {
            1 => Some(Self::Add),
            2 => Some(Self::Multiply),
            _ => None,
        }
    }
}

struct IntCode {}

impl IntCode {
    fn execute(program: &mut Vec<i32>) {
        let mut pos = 0;
        loop {
            let opcode = program[pos];
            if opcode == 99 {
                break;
            }
            let idx1 = program[pos + 1] as usize;
            let idx2 = program[pos + 2] as usize;
            let idx3 = program[pos + 3] as usize;
            let in1 = program[idx1];
            let in2 = program[idx2];
            match OpCode::from(opcode) {
                Some(OpCode::Add) => program[idx3] = in1 + in2,
                Some(OpCode::Multiply) => program[idx3] = in1 * in2,
                _ => panic!("invalid opcode"),
            }
            pos += 4;
        }
    }
}

#[test]
fn puzzle_one_example_one() {
    let mut program = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
    let result = vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50];
    IntCode::execute(&mut program);
    assert_eq!(result, program);
}

#[test]
fn puzzle_one_example_two() {
    let mut program = vec![1, 0, 0, 0, 99];
    let result = vec![2, 0, 0, 0, 99];
    IntCode::execute(&mut program);
    assert_eq!(result, program);
}

#[test]
fn puzzle_one_example_three() {
    let mut program = vec![2, 3, 0, 3, 99];
    let result = vec![2, 3, 0, 6, 99];
    IntCode::execute(&mut program);
    assert_eq!(result, program);
}

#[test]
fn puzzle_one_example_four() {
    let mut program = vec![2, 4, 4, 5, 99, 0];
    let result = vec![2, 4, 4, 5, 99, 9801];
    IntCode::execute(&mut program);
    assert_eq!(result, program);
}

#[test]
fn puzzle_one_example_five() {
    let mut program = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
    let result = vec![30, 1, 1, 4, 2, 5, 6, 0, 99];
    IntCode::execute(&mut program);
    assert_eq!(result, program);
}
