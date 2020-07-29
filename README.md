# Advent of Code

Advent of Code is an annual Advent calendar of small programming puzzles for a
variety of skill sets and skill levels that can be solved in any programming
language you like.

## Events

- [AoC 2015: https://adventofcode.com/2015](https://adventofcode.com/2015)
- [AoC 2016: https://adventofcode.com/2016](https://adventofcode.com/2016)
- [AoC 2017: https://adventofcode.com/2017](https://adventofcode.com/2017)
- [AoC 2018: https://adventofcode.com/2018](https://adventofcode.com/2018)
- [AoC 2019: https://adventofcode.com/2019](https://adventofcode.com/2019)

## Solutions

This repository is a collection of possible solution to each puzzle using the
[Rust](https://www.rust-lang.org/) programming language.

- [Event 2015](aoc_2015/README.md)
- [Event 2016](aoc_2016/README.md)
- [Event 2017](aoc_2017/README.md)
- [Event 2018](aoc_2018/README.md)
- [Event 2019](aoc_2019/README.md)

## Usage

```shell-session
# clone repository
$ git clone https://github.com/logansquirel/Advent_of_Code.git
# select year and day
$ cd aoc_yyyy/day_dd
# run puzzle
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

## Questions, Issues, Contributing

For questions and issues, open an issue
[here](https://github.com/logansquirel/Advent_of_Code/issues).

Contributions and Pull Requests (PR) are welcome.

## License

[MIT License](https://choosealicense.com/licenses/mit/)
