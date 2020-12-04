pub fn puzzle_one(input: &str) -> String {
    let mut code = String::new();
    let mut pos = BasicKeyPad::new(5);
    for line in input.trim().lines() {
        for char in line.trim().chars() {
            match char {
                'U' => pos.up(),
                'D' => pos.down(),
                'L' => pos.left(),
                'R' => pos.right(),
                _ => continue,
            }
        }
        code.push_str(&pos.pos.to_string())
    }
    code
}

pub fn puzzle_two(input: &str) -> String {
    let mut code = String::new();
    let mut pos = AdvancedKeyPad::new('5');
    for line in input.trim().lines() {
        for char in line.trim().chars() {
            match char {
                'U' => pos.up(),
                'D' => pos.down(),
                'L' => pos.left(),
                'R' => pos.right(),
                _ => continue,
            }
        }
        code.push_str(&pos.pos.to_string())
    }
    code
}

#[derive(Debug)]
struct BasicKeyPad {
    pos: u32,
}

impl BasicKeyPad {
    pub fn new(pos: u32) -> Self {
        BasicKeyPad { pos }
    }

    pub fn up(&mut self) {
        self.pos = match self.pos {
            1 => 1,
            2 => 2,
            3 => 3,
            4 => 1,
            5 => 2,
            6 => 3,
            7 => 4,
            8 => 5,
            9 => 6,
            _ => self.pos,
        }
    }

    pub fn down(&mut self) {
        self.pos = match self.pos {
            1 => 4,
            2 => 5,
            3 => 6,
            4 => 7,
            5 => 8,
            6 => 9,
            7 => 7,
            8 => 8,
            9 => 9,
            _ => self.pos,
        }
    }

    pub fn left(&mut self) {
        self.pos = match self.pos {
            1 => 1,
            2 => 1,
            3 => 2,
            4 => 4,
            5 => 4,
            6 => 5,
            7 => 7,
            8 => 7,
            9 => 8,
            _ => self.pos,
        }
    }

    pub fn right(&mut self) {
        self.pos = match self.pos {
            1 => 2,
            2 => 3,
            3 => 3,
            4 => 5,
            5 => 6,
            6 => 6,
            7 => 8,
            8 => 9,
            9 => 9,
            _ => self.pos,
        }
    }
}

struct AdvancedKeyPad {
    pos: char,
}

impl AdvancedKeyPad {
    pub fn new(pos: char) -> Self {
        AdvancedKeyPad { pos }
    }

    pub fn up(&mut self) {
        self.pos = match self.pos {
            '1' => '1',
            '2' => '2',
            '3' => '1',
            '4' => '4',
            '5' => '5',
            '6' => '2',
            '7' => '3',
            '8' => '4',
            '9' => '9',
            'A' => '6',
            'B' => '7',
            'C' => '8',
            'D' => 'B',
            _ => self.pos,
        }
    }

    pub fn down(&mut self) {
        self.pos = match self.pos {
            '1' => '3',
            '2' => '6',
            '3' => '7',
            '4' => '8',
            '5' => '5',
            '6' => 'A',
            '7' => 'B',
            '8' => 'C',
            '9' => '9',
            'A' => 'A',
            'B' => 'D',
            'C' => 'C',
            'D' => 'D',
            _ => self.pos,
        }
    }

    pub fn left(&mut self) {
        self.pos = match self.pos {
            '1' => '1',
            '2' => '2',
            '3' => '2',
            '4' => '3',
            '5' => '5',
            '6' => '5',
            '7' => '6',
            '8' => '7',
            '9' => '8',
            'A' => 'A',
            'B' => 'A',
            'C' => 'B',
            'D' => 'D',
            _ => self.pos,
        }
    }

    pub fn right(&mut self) {
        self.pos = match self.pos {
            '1' => '1',
            '2' => '3',
            '3' => '4',
            '4' => '4',
            '5' => '6',
            '6' => '7',
            '7' => '8',
            '8' => '9',
            '9' => '9',
            'A' => 'B',
            'B' => 'C',
            'C' => 'C',
            'D' => 'D',
            _ => self.pos,
        }
    }
}

#[test]
fn puzzle_one_example() {
    assert_eq!(String::from("1985"), puzzle_one("ULL\nRRDDD\nLURDL\nUUUUD"));
}

#[test]
fn puzzle_two_example() {
    assert_eq!(String::from("5DB3"), puzzle_two("ULL\nRRDDD\nLURDL\nUUUUD"));
}
