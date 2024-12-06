use std::time::Instant;

use anyhow::bail;
use common::{
    cartesian::{matrix_from_lines, Point, ScreenDir},
    OptionAnyhow,
};
use nalgebra::DMatrix;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Block {
    Empty,
    Wall,
}
impl Default for Block {
    fn default() -> Self {
        Self::Empty
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Guard(Point, ScreenDir);

type Map = DMatrix<Block>;

#[derive(Debug, Clone)]
pub struct Problem {
    map: Map,
    guard: Guard,
}

#[derive(Debug, Clone, Copy)]
enum Termination {
    Exited,
    Loop,
}

fn parse_input(input: &str) -> anyhow::Result<Problem> {
    let lines: Vec<_> = input.lines().collect();

    // load map
    let mut guard = None;
    let map = matrix_from_lines(&lines, |ch| match ch {
        '.' => Ok(Block::Empty),
        '^' => Ok(Block::Empty),
        '#' => Ok(Block::Wall),
        _ => bail!("unexpected map character: {}", ch),
    })?;

    // locate guard - planning on refactoring above later, so keeping this separate
    for (row, line) in lines.iter().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            if ch == '^' {
                let (y, x) = (row as i64, col as i64);
                guard = Some(Guard(Point::new(x, y), ScreenDir::U));
            }
        }
    }

    Ok(Problem {
        map,
        guard: guard.ok_anyhow()?,
    })
}

fn part1(problem: &Problem) -> usize {
    let mut guard = problem.guard;

    let mut visited = DMatrix::from_element(problem.map.nrows(), problem.map.ncols(), false);
    visited[guard.0.to_coord().unwrap()] = true;
    loop {
        let next_pos = guard.0 + Point::from(guard.1);

        if let Some(coord) = next_pos.to_coord_matrix(&problem.map) {
            let block = problem.map[coord];
            if block == Block::Empty {
                visited[coord] = true;
                guard.0 = next_pos;
            } else {
                guard.1 = guard.1.right();
            }
        } else {
            break;
        }
    }

    visited.iter().filter(|v| **v).count()
}

fn part2(problem: &Problem) -> usize {
    let mut loop_termination_count = 0;
    let mut problem_temp = problem.clone();
    let mut visited = DMatrix::from_element(problem.map.nrows(), problem.map.ncols(), None);
    for c in 0..problem.map.ncols() {
        for r in 0..problem.map.nrows() {
            if problem.map[(r, c)] == Block::Empty {
                // insert temporary block
                problem_temp.map.copy_from(&problem.map);
                problem_temp.map[(r, c)] = Block::Wall;

                // iterate and check for infinite loop
                if let Termination::Loop = iterate(&problem_temp, &mut visited) {
                    loop_termination_count += 1
                }
            }
        }
    }
    loop_termination_count
}

fn iterate(problem: &Problem, visited: &mut DMatrix<Option<ScreenDir>>) -> Termination {
    let mut guard = problem.guard;
    visited.fill(None);
    visited[guard.0.to_coord().unwrap()] = Some(guard.1);
    loop {
        let next_pos = guard.0 + Point::from(guard.1);
        if let Some(coord) = next_pos.to_coord_matrix(&problem.map) {
            let block = problem.map[coord];
            if block == Block::Empty {
                // same location, same direction - we're in a loop
                if let Some(dir) = visited[coord] {
                    if dir == guard.1 {
                        return Termination::Loop;
                    }
                }
                visited[coord] = Some(guard.1);

                // update position
                guard.0 = next_pos;
            } else {
                guard.1 = guard.1.right();
            }
        } else {
            break;
        }
    }
    Termination::Exited
}

fn main() -> anyhow::Result<()> {
    let text = common::read_file("input1.txt")?;

    let problem = parse_input(&text)?;

    let t1 = Instant::now();
    let count_part1 = part1(&problem);
    println!("Part 1 count is {count_part1} in {:?}", t1.elapsed());

    let t2 = Instant::now();
    let count_part2 = part2(&problem);
    println!("Part 2 count is {count_part2} in {:?}", t2.elapsed());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
        ....#.....
        .........#
        ..........
        ..#.......
        .......#..
        ..........
        .#..^.....
        ........#.
        #.........
        ......#...
    "};

    #[test]
    fn test_parse_input() {
        let problem = parse_input(EXAMPLE).unwrap();
        println!("{:?}", problem);
    }

    #[test]
    fn part1_correct() {
        let problem = parse_input(EXAMPLE).unwrap();
        let count = part1(&problem);
        assert_eq!(count, 41);
    }

    #[test]
    fn part2_correct() {
        let problem = parse_input(EXAMPLE).unwrap();
        let count = part2(&problem);
        assert_eq!(count, 6);
    }
}
