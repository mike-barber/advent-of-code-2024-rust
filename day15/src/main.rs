use anyhow::{bail, Result};
use common::cartesian::{matrix_from_lines, Point, ScreenDir};
use nalgebra::DMatrix;
use std::time::Instant;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
enum Block {
    #[default]
    Open,
    Box,
    Wall,
}

type Map = DMatrix<Block>;
type Instructions = Vec<ScreenDir>;

#[derive(Debug, Clone)]
pub struct Problem {
    map: Map,
    robot: Point,
    instructions: Instructions,
}

fn parse_input(input: &str) -> Result<Problem> {
    let mut lines_iter = input.lines();

    let map_lines: Vec<_> = (&mut lines_iter).take_while(|l| !l.is_empty()).collect();

    // parse map
    let mut robot = Point::default();
    let rows = map_lines.len();
    let cols = map_lines.iter().map(|l| l.chars().count()).max().unwrap();
    let mut map = DMatrix::from_element(rows, cols, Block::default());
    for (r, line) in map_lines.iter().enumerate() {
        for (c, ch) in line.chars().enumerate() {
            let block = match ch {
                '#' => Block::Wall,
                'O' => Block::Box,
                '.' => Block::Open,
                '@' => {
                    robot = Point::new(c as i64, r as i64);
                    Block::Open
                }
                _ => bail!("Unknown block type {}", ch),
            };
        }
    }

    // parse instructions
    let mut instructions = Vec::new();
    for l in lines_iter {
        for ch in l.chars() {
            instructions.push(match ch {
                '<' => ScreenDir::L,
                '>' => ScreenDir::R,
                'v' => ScreenDir::D,
                '^' => ScreenDir::U,
                _ => bail!("Unknown instruction {}", ch),
            });
        }
    }

    Ok(Problem {
        map,
        robot,
        instructions,
    })
}

fn part1(problem: &Problem) -> Result<usize> {
    Ok(1)
}

fn part2(problem: &Problem) -> Result<usize> {
    Ok(2)
}

fn main() -> anyhow::Result<()> {
    let text = common::read_file("input1.txt")?;
    let problem = parse_input(&text)?;

    let t1 = Instant::now();
    let count_part1 = part1(&problem)?;
    println!("Part 1 result is {count_part1} (took {:?})", t1.elapsed());

    let t2 = Instant::now();
    let count_part2 = part2(&problem)?;
    println!("Part 2 result is {count_part2} (took {:?})", t2.elapsed());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_parse_input() -> Result<()> {
        let problem = parse_input(EXAMPLE_SMALL)?;
        println!("{:?}", problem);
        Ok(())
    }

    #[test]
    fn part1_correct() -> Result<()> {
        let problem = parse_input(EXAMPLE_SMALL)?;
        let count = part1(&problem)?;
        assert_eq!(count, 1);
        Ok(())
    }

    #[test]
    fn part2_correct() -> Result<()> {
        let problem = parse_input(EXAMPLE_SMALL)?;
        let count = part2(&problem)?;
        assert_eq!(count, 2);
        Ok(())
    }

    const EXAMPLE_SMALL: &str = indoc! {"
        ########
        #..O.O.#
        ##@.O..#
        #...O..#
        #.#.O..#
        #...O..#
        #......#
        ########

        <^^>>>vv<v>>v<<
    "};

    const EXAMPLE: &str = indoc! {"
        ##########
        #..O..O.O#
        #......O.#
        #.OO..O.O#
        #..O@..O.#
        #O#..O...#
        #O..O..O.#
        #.OO.O.OO#
        #....O...#
        ##########

        <vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
        vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
        ><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
        <<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
        ^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
        ^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
        >^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
        <><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
        ^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
        v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
    "};
}
