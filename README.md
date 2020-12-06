# 🎄 Advent of Code 🎅

Advent of Code is an annual Advent calendar of small programming puzzles for a
variety of skill sets and skill levels that can be solved in any programming
language you like.

## Events

[![AoC-2015](https://img.shields.io/badge/Advent_of_Code-2015-d08770?style=for-the-badge)](https://adventofcode.com/2015)

[![AoC-2016](https://img.shields.io/badge/Advent_of_Code-2016-d08770?style=for-the-badge)](https://adventofcode.com/2016)

[![AoC-2017](https://img.shields.io/badge/Advent_of_Code-2017-d08770?style=for-the-badge)](https://adventofcode.com/2017)

[![AoC-2018](https://img.shields.io/badge/Advent_of_Code-2018-d08770?style=for-the-badge)](https://adventofcode.com/2018)

[![AoC-2019](https://img.shields.io/badge/Advent_of_Code-2019-d08770?style=for-the-badge)](https://adventofcode.com/2019)

[![AoC-2020](https://img.shields.io/badge/Advent_of_Code-2020-d08770?style=for-the-badge)](https://adventofcode.com/2020)

## Solutions

[![CI](https://img.shields.io/github/workflow/status/logansquirel/advent_of_code/CI/master?color=b48ead&label=CI&logo=github&style=for-the-badge)](https://github.com/logansquirel/advent_of_code/actions?query=branch%3Amaster)

This repository is a collection of possible solution to each puzzle using the
[Rust](https://www.rust-lang.org/) programming language.

[![AoC-2015](https://img.shields.io/badge/AoC--2015-6%20%E2%98%85-5e81ac?style=for-the-badge)](aoc_2015/README.md)

[![AoC-2016](https://img.shields.io/badge/AoC--2016-6%20%E2%98%85-5e81ac?style=for-the-badge)](aoc_2016/README.md)

[![AoC-2017](https://img.shields.io/badge/AoC--2017-6%20%E2%98%85-5e81ac?style=for-the-badge)](aoc_2017/README.md)

[![AoC-2018](https://img.shields.io/badge/AoC--2018-4%20%E2%98%85-5e81ac?style=for-the-badge)](aoc_2018/README.md)

[![AoC-2019](https://img.shields.io/badge/AoC--2019-4%20%E2%98%85-5e81ac?style=for-the-badge)](aoc_2019/README.md)

[![AoC-2020](https://img.shields.io/badge/AoC--2020-0%20%E2%98%85-5e81ac?style=for-the-badge)](aoc_2020/README.md)

## Usage

```console
$ git clone https://github.com/logansquirel/advent_of_code.git
$ cd aoc_yyyy/day_dd
$ cargo run --quiet --release < input/input.dat
Advent of Code yyyy-dd
------ Puzzle 1 ------
Answer 1

------ Puzzle 2 ------
Answer 2
```

- Replace `yyyy` with the four digits year
- Replace `dd` with the two digits day (0-padded)
- (Optional) Replace the path `input/input.dat` with the path to your personal
  puzzle input.

## Template

This repository provides a [Rust](https://www.rust-lang.org/) template
(`aoc_yyyy/day_dd`) for advent of code.

```console
$ tree aoc_yyyy
aoc_yyyy
└── day_dd
    ├── Cargo.toml              # Cargo configuration file
    ├── input
    │   └── input.dat           # Puzzle input
    ├── readme.md
    ├── src
    │   ├── lib.rs              # Puzzle solutions and unit tests
    │   └── main.rs             # Puzzle main
    └── tests
        └── aoc_yyyy_day_dd.rs  # Puzzle answers tests

4 directories, 6 files
```

- Replace all `yyyy` occurences with the four digits year
- Replace all `dd` occurences with the two digits day (0-padded)
- Implement solution in `lib.rs`
- Verify answers in `aoc_yyyy_day_dd.rs`

## Questions, Issues, Contributing

For questions and issues, open an issue
[here](https://github.com/logansquirel/Advent_of_Code/issues).

Contributions and Pull Requests (PR) are welcome.

## License

[![License](https://img.shields.io/badge/license-mit-81a1c1?style=for-the-badge)](LICENSE)
