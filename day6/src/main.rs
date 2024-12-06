use std::collections::HashSet;

use anyhow::bail;
use common::OptionAnyhow;
use day6::{Point, ScreenDir};
use nalgebra::DMatrix;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Block {
    Empty,
    Wall,
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

    let rows = lines.len();
    let cols = lines.iter().map(|l| l.chars().count()).max().ok_anyhow()?;

    // load map
    let mut map = DMatrix::from_element(rows, cols, Block::Empty);
    let mut guard = None;
    for row in 0..rows {
        let line = lines[row];
        for (col, ch) in line.chars().enumerate() {
            let block_type = match ch {
                '.' => Block::Empty,
                '^' => Block::Empty,
                '#' => Block::Wall,
                _ => bail!("unexpected map character: {}", ch),
            };
            map[(row, col)] = block_type;
        }
    }

    // locate guard - planning on refactoring above later, so keeping this separate
    for row in 0..rows {
        let line = lines[row];
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

    let mut visited = HashSet::new();
    visited.insert(guard.0);
    loop {
        let next_pos = guard.0 + Point::from(guard.1);

        if let Some(coord) = next_pos.to_coord_matrix(&problem.map) {
            let block = problem.map[coord];
            if block == Block::Empty {
                visited.insert(next_pos);
                guard.0 = next_pos;
            } else {
                guard.1 = guard.1.right();
            }
        } else {
            break;
        }
    }

    visited.len()
}

fn part2(problem: &Problem) -> usize {
    let mut loop_termination_count = 0;
    let mut problem_temp = problem.clone();
    let mut visited = HashSet::new();
    for c in 0..problem.map.ncols() {
        for r in 0..problem.map.nrows() {
            if problem.map[(r, c)] == Block::Empty {
                // insert temporary block
                problem_temp.map.copy_from(&problem.map);
                problem_temp.map[(r, c)] = Block::Wall;
                
                // iterate and check for infinite loop
                match iterate(&problem_temp, &mut visited) {
                    Termination::Loop => loop_termination_count += 1,
                    _ => (),
                }
            }
        }
    }
    loop_termination_count
}

fn iterate(problem: &Problem, visited: &mut HashSet<Guard>) -> Termination {
    let mut guard = problem.guard;
    visited.clear();
    visited.insert(guard);
    loop {
        let next_pos = guard.0 + Point::from(guard.1);
        if let Some(coord) = next_pos.to_coord_matrix(&problem.map) {
            let block = problem.map[coord];
            if block == Block::Empty {
                guard.0 = next_pos;
            } else {
                guard.1 = guard.1.right();
            }

            if visited.contains(&guard) {
                return Termination::Loop;
            } else {
                visited.insert(guard);
            }
        } else {
            break;
        }
    }
    
    return Termination::Exited
}


fn main() -> anyhow::Result<()> {
    let text = common::read_file("input1.txt")?;

    let problem = parse_input(&text)?;

    let count_part1 = part1(&problem);
    println!("Part 1 count is {count_part1}");

    let count_part2 = part2(&problem);
    println!("Part 2 count is {count_part2}");

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
