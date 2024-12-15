use anyhow::{bail, Result};
use common::cartesian::{Point, ScreenDir};
use nalgebra::DMatrix;
use std::{
    collections::{HashMap, HashSet},
    iter,
    time::Instant,
};

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
enum Block {
    #[default]
    Open,
    // whole box for part 1
    BoxWhole,
    // left or right part of box for part 2
    BoxL,
    BoxR,
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
impl std::fmt::Display for Problem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for r in 0..self.map.nrows() {
            for c in 0..self.map.ncols() {
                let p = Point::new(c as i64, r as i64);
                let b = self.map.get(p).unwrap();
                if p == self.robot {
                    write!(f, "@")?;
                    assert!(*b == Block::Open);
                } else {
                    let ch = match *b {
                        Block::Open => ".",
                        Block::BoxWhole => "O",
                        Block::BoxL => "[",
                        Block::BoxR => "]",
                        Block::Wall => "#",
                    };
                    write!(f, "{}", ch)?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
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
                'O' => Block::BoxWhole,
                '.' => Block::Open,
                '@' => {
                    robot = Point::new(c as i64, r as i64);
                    Block::Open
                }
                _ => bail!("Unknown block type {}", ch),
            };
            map[(r, c)] = block;
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

fn dir_iter(loc: Point, dir: ScreenDir) -> impl Iterator<Item = Point> {
    let dir_pt = dir.into();
    iter::successors(Some(loc + dir_pt), move |p| Some(*p + dir_pt))
}

impl Problem {
    fn gps_score(&self) -> usize {
        let mut score = 0;
        for r in 0..self.map.nrows() {
            for c in 0..self.map.ncols() {
                score += match self.map[(r, c)] {
                    Block::BoxWhole | Block::BoxL => 100 * r + c,
                    _ => 0,
                }
            }
        }
        score
    }

    fn move_robot_part_1(&mut self, dir: ScreenDir) -> Option<usize> {
        let p = self.robot;

        let num_boxes = dir_iter(p, dir)
            .map(|p| self.map.get(p))
            .take_while(|b| b.copied() == Some(Block::BoxWhole))
            .count();

        let loc_after_boxes = dir_iter(p, dir).nth(num_boxes)?;
        let block_after_boxes = self.map.get(loc_after_boxes).copied()?;
        if block_after_boxes != Block::Open {
            return None;
        }

        // move the whole chain
        if num_boxes > 0 {
            *self.map.get_mut(loc_after_boxes).unwrap() = Block::BoxWhole;
        }
        let robot_next = dir_iter(p, dir).nth(0).unwrap();
        *self.map.get_mut(robot_next).unwrap() = Block::Open;
        self.robot = robot_next;

        Some(num_boxes)
    }

    fn move_robot_part_2(&mut self, dir: ScreenDir) -> Option<usize> {
        let p = self.robot;
        let dp: Point = dir.into();

        let mut move_set: HashMap<Point, Block> = HashMap::new();
        let mut to_visit = Vec::new();
        let mut visited = HashSet::new();

        // build set of affected boxes
        to_visit.push(self.robot + dp);
        while let Some(p) = to_visit.pop() {
            if visited.contains(&p) {
                continue;
            }

            let b = *self.map.get(p).unwrap();

            // collision with wall - no move possible
            if b == Block::Wall {
                return None;
            }

            match dir {
                // left-right
                ScreenDir::L | ScreenDir::R => match b {
                    Block::BoxL | Block::BoxR => {
                        move_set.insert(p, b);
                        to_visit.push(p + dp);
                    }
                    Block::Open => {}
                    _ => {
                        panic!("unexpected block {:?}", b);
                    }
                },
                // up-down
                ScreenDir::U | ScreenDir::D => match b {
                    Block::BoxL => {
                        let other_side_box = Point::new(1, 0);
                        move_set.insert(p, b);
                        move_set.insert(p + other_side_box, Block::BoxR);
                        to_visit.push(p + dp);
                        to_visit.push(p + other_side_box + dp);
                    }
                    Block::BoxR => {
                        let other_side_box = Point::new(-1, 0);
                        move_set.insert(p, b);
                        move_set.insert(p + other_side_box, Block::BoxL);
                        to_visit.push(p + dp);
                        to_visit.push(p + other_side_box + dp);
                    }
                    Block::Open => {}
                    _ => {
                        panic!("unexpected block {:?}", b);
                    }
                },
            }
            visited.insert(p);
        }

        // at this point, we know we have no collisions with walls; move the whole thing
        // 1. clear map
        for (p, _b) in move_set.iter() {
            *self.map.get_mut(*p).unwrap() = Block::Open;
        }
        // 2. place boxes in new location
        for (p, b) in move_set.iter() {
            let p = *p + dp;
            *self.map.get_mut(p).unwrap() = *b;
        }
        // 3. update robot position
        self.robot = p + dp;

        Some(move_set.len())
    }

    fn to_part_2_problem(&self) -> Result<Self> {
        let mut new_map =
            DMatrix::from_element(self.map.nrows(), self.map.ncols() * 2, Block::Open);

        for r in 0..self.map.nrows() {
            for c in 0..self.map.ncols() {
                let (left, right) = match self.map[(r, c)] {
                    Block::Open => (Block::Open, Block::Open),
                    Block::BoxWhole => (Block::BoxL, Block::BoxR),
                    Block::BoxL => bail!("part 1 map should not contain BoxL"),
                    Block::BoxR => bail!("part 1 map should not contain BoxR"),
                    Block::Wall => (Block::Wall, Block::Wall),
                };
                new_map[(r, 2 * c)] = left;
                new_map[(r, 2 * c + 1)] = right;
            }
        }

        Ok(Problem {
            map: new_map,
            instructions: self.instructions.clone(),
            robot: self.robot * Point::new(2, 1),
        })
    }
}

fn part1(problem: &Problem) -> Result<usize> {
    let mut problem = problem.clone();
    let instructions = problem.instructions.clone();

    for inst in instructions {
        problem.move_robot_part_1(inst);
    }
    println!("{}", problem);

    let score = problem.gps_score();
    Ok(score)
}

fn part2(problem: &Problem) -> Result<usize> {
    let mut problem = problem.to_part_2_problem()?;
    let instructions = problem.instructions.clone();

    for inst in instructions {
        problem.move_robot_part_2(inst);
    }
    println!("{}", problem);

    let score = problem.gps_score();
    Ok(score)
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
    fn part1_small_correct() -> Result<()> {
        let problem = parse_input(EXAMPLE_SMALL)?;
        let count = part1(&problem)?;
        assert_eq!(count, 2028);
        Ok(())
    }

    #[test]
    fn part1_correct() -> Result<()> {
        let problem = parse_input(EXAMPLE)?;
        let count = part1(&problem)?;
        assert_eq!(count, 10092);
        Ok(())
    }

    #[test]
    fn part2_small_correct() -> Result<()> {
        let problem = parse_input(EXAMPLE_SMALL_PART2)?;
        let count = part2(&problem)?;
        assert_eq!(count, 618);
        Ok(())
    }

    #[test]
    fn part2_correct() -> Result<()> {
        let problem = parse_input(EXAMPLE)?;
        let count = part2(&problem)?;
        assert_eq!(count, 9021);
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

    const EXAMPLE_SMALL_PART2: &str = indoc! {"
        #######
        #...#.#
        #.....#
        #..OO@#
        #..O..#
        #.....#
        #######

        <vv<<^^<<^^
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
