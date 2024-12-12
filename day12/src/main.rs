use std::time::Instant;

use anyhow::Result;
use common::cartesian::{matrix_from_lines, Point, ScreenDir};
use nalgebra::DMatrix;
use strum::IntoEnumIterator;

type PlantMap = DMatrix<char>;
type RegionMap = DMatrix<i32>;

#[derive(Debug, Clone)]
pub struct Measurement {
    area: usize,
    perimeter: usize,
    sides: usize,
}

#[derive(Debug, Clone)]
pub struct Problem {
    plants: PlantMap,
}
impl Problem {
    fn perimeter(&self, loc: Point) -> usize {
        let mut perim = 0;
        let ch = self.plants.get(loc).unwrap();
        for n in neighbours(loc) {
            perim += match self.plants.get(n) {
                None => 1,
                Some(nch) if nch == ch => 0,
                Some(_) => 1,
            }
        }
        perim
    }

    fn border(&self, loc: Point, d: ScreenDir) -> bool {
        let next = loc + d.into();
        let ch = self.plants.get(loc).unwrap();
        match self.plants.get(next) {
            Some(nch) if nch == ch => false,
            Some(_) => true, // different plant
            None => true,    // edge of
        }
    }

    fn corners(&self, loc: Point) -> usize {
        let mut inside_corners = 0;
        let mut outside_corners = 0;

        let ch = self.plants.get(loc).unwrap();
        let diag_different_plant = |d1: ScreenDir, d2: ScreenDir| {
            let next = loc + d1.into() + d2.into();
            if let Some(nch) = self.plants.get(next) {
                nch != ch
            } else {
                false // not on map
            }
        };

        for d in ScreenDir::iter() {
            let adjacent = d.right();

            let border = self.border(loc, d);
            let adjacent_border = self.border(loc, adjacent);
            let diag = diag_different_plant(d, adjacent);

            if border && adjacent_border {
                outside_corners += 1;
            } else if !border && !adjacent_border && diag {
                inside_corners += 1;
            }
        }
        inside_corners + outside_corners
    }

    fn explore_region(&self, loc: Point, regions: &mut RegionMap, label: i32) -> Measurement {
        let mut area = 0;
        let mut perimeter = 0;
        let mut corners = 0;

        let mut queue = Vec::new();
        queue.push(loc);

        let plant = self.plants.get(loc).unwrap();
        loop {
            // explore next location
            let current = match queue.pop() {
                None => break,
                Some(n) => n,
            };

            // only if not visited
            if *regions.get(current).unwrap() != -1 {
                continue;
            }

            // add area & record visited
            area += 1;
            perimeter += self.perimeter(current);
            corners += self.corners(current);
            *regions.get_mut(current).unwrap() = label;
            //println!("{current:?} {area}");

            // find possible neighbours
            for next in neighbours(current) {
                // only unexplored
                if let Some(r) = regions.get(next) {
                    if *r != -1 {
                        continue;
                    }
                }
                // and only if it matches our plant type
                if let Some(ch) = self.plants.get(next) {
                    if ch == plant {
                        queue.push(next);
                    }
                }
            }
        }

        Measurement {
            area,
            perimeter,
            sides: corners,
        }
    }
}

fn neighbours(loc: Point) -> impl Iterator<Item = Point> {
    ScreenDir::iter().map(move |d| loc + d.into())
}

fn parse_input(input: &str) -> Result<Problem> {
    let lines: Vec<_> = input.lines().collect();
    let plants = matrix_from_lines(&lines, Ok)?;
    Ok(Problem { plants })
}

fn part1(problem: &Problem) -> Result<usize> {
    let mut total_price = 0;
    let mut region_map =
        RegionMap::from_element(problem.plants.nrows(), problem.plants.ncols(), -1);

    let mut label = 0;
    for x in 0..problem.plants.ncols() {
        for y in 0..problem.plants.nrows() {
            let loc = Point::new(x as i64, y as i64);
            if *region_map.get(loc).unwrap() == -1 {
                // unexplored -- map this region
                let measurement = problem.explore_region(loc, &mut region_map, label);
                // println!("{loc:?} {measurement:?}");
                // println!("{region_map}");
                label += 1;
                total_price += measurement.area * measurement.perimeter
            }
        }
    }

    Ok(total_price)
}

fn part2(problem: &Problem) -> Result<usize> {
    let mut total_price = 0;
    let mut region_map =
        RegionMap::from_element(problem.plants.nrows(), problem.plants.ncols(), -1);

    let mut label = 0;
    for x in 0..problem.plants.ncols() {
        for y in 0..problem.plants.nrows() {
            let loc = Point::new(x as i64, y as i64);
            if *region_map.get(loc).unwrap() == -1 {
                // unexplored -- map this region
                let measurement = problem.explore_region(loc, &mut region_map, label);
                //println!("{loc:?} {measurement:?}");
                // println!("{region_map}");
                label += 1;
                total_price += measurement.area * measurement.sides
            }
        }
    }

    Ok(total_price)
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
        RRRRIICCFF
        RRRRIICCCF
        VVRRRCCFFF
        VVRCCCJFFF
        VVVVCJJCFE
        VVIVCCJJEE
        VVIIICJJEE
        MIIIIIJJEE
        MIIISIJEEE
        MMMISSJEEE
    "};

    #[test]
    fn test_parse_input() -> Result<()> {
        let problem = parse_input(EXAMPLE)?;
        println!("{:?}", problem);
        Ok(())
    }

    #[test]
    fn part1_correct() -> Result<()> {
        let problem = parse_input(EXAMPLE)?;
        let count = part1(&problem)?;
        assert_eq!(count, 1930);
        Ok(())
    }

    #[test]
    fn part2_correct() -> Result<()> {
        let problem = parse_input(EXAMPLE)?;
        let count = part2(&problem)?;
        assert_eq!(count, 1206);
        Ok(())
    }
}
