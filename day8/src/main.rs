use anyhow::Result;
use common::cartesian::{matrix_from_lines, Point};
use itertools::Itertools;
use nalgebra::DMatrix;
use std::{collections::HashMap, iter::successors, time::Instant};

type AntennaMap = DMatrix<AntennaElement>;
type AntinodeMap = DMatrix<AntinodeElement>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct AntinodeElement(bool);
impl std::fmt::Display for AntinodeElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            true => write!(f, "#"),
            false => write!(f, "."),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AntennaElement {
    None,
    Antenna(char),
}
impl Default for AntennaElement {
    fn default() -> Self {
        Self::None
    }
}
impl std::fmt::Display for AntennaElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AntennaElement::None => write!(f, "."),
            AntennaElement::Antenna(ch) => write!(f, "{}", ch),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Problem {
    map: AntennaMap,
}

fn parse_input(input: &str) -> Result<Problem> {
    let lines: Vec<_> = input.lines().collect();

    let map = matrix_from_lines(&lines, |ch| match ch {
        '.' => Ok(AntennaElement::None),
        ch => Ok(AntennaElement::Antenna(ch)),
    })?;
    Ok(Problem { map })
}

fn count_antinodes(problem: &Problem, exclude_antenna: bool, harmonics: usize) -> Result<usize> {
    let map = &problem.map;
    let mut antinodes = AntinodeMap::from_element(map.nrows(), map.ncols(), AntinodeElement(false));

    // group all antenna types
    let mut antennae = HashMap::new();
    for r in 0..map.nrows() {
        for c in 0..map.ncols() {
            if let AntennaElement::Antenna(freq) = map[(r, c)] {
                let entry = antennae.entry(freq).or_insert(Vec::new());
                let point = Point::from((r, c));
                entry.push(point);
            }
        }
    }

    // iterate through all pairs
    for (_, list) in antennae {
        for pair in list.iter().copied().combinations(2) {
            let a = pair[0];
            let b = pair[1];
            let delta = a - b;

            let init_offset = match exclude_antenna {
                true => delta,
                false => Point::default(),
            };

            // iterate through harmonics until we run off the map or number required
            for coord in successors(Some(a + init_offset), |p| Some(*p + delta))
                .take(harmonics)
                .filter_map(|pt| pt.to_coord_matrix(map))
            {
                antinodes[coord] = AntinodeElement(true);
            }

            for coord in successors(Some(b - init_offset), |p| Some(*p - delta))
                .take(harmonics)
                .filter_map(|pt| pt.to_coord_matrix(map))
            {
                antinodes[coord] = AntinodeElement(true);
            }
        }
    }

    // count antinodes on map
    let num_antinodes = antinodes.iter().filter(|n| n.0).count();

    // println!("{}", map);
    // println!("{}", antinodes);

    Ok(num_antinodes)
}

fn part1(problem: &Problem) -> Result<usize> {
    count_antinodes(problem, true, 1)
}

fn part2(problem: &Problem) -> Result<usize> {
    let max_harmonics = problem.map.nrows().max(problem.map.ncols());
    count_antinodes(problem, false, max_harmonics)
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

    const EXAMPLE: &str = indoc! {"
        ............
        ........0...
        .....0......
        .......0....
        ....0.......
        ......A.....
        ............
        ............
        ........A...
        .........A..
        ............
        ............
    "};

    #[test]
    fn test_parse_input() -> Result<()> {
        let problem = parse_input(EXAMPLE)?;
        println!("{}", problem.map);
        Ok(())
    }

    #[test]
    fn part1_correct() -> Result<()> {
        let problem = parse_input(EXAMPLE)?;
        let count = part1(&problem)?;
        assert_eq!(count, 14);
        Ok(())
    }

    #[test]
    fn part2_correct() -> Result<()> {
        let problem = parse_input(EXAMPLE)?;
        let count = part2(&problem)?;
        assert_eq!(count, 34);
        Ok(())
    }
}
