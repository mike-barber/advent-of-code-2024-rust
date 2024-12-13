use std::time::Instant;

use anyhow::Result;
use common::{cartesian::Point, OptionAnyhow};
use regex::Regex;

#[derive(Debug, Clone)]
pub struct Problem {
    machines: Vec<Machine>,
}
const A_COST: i64 = 3;
const B_COST: i64 = 1;
const PART2_OFFSET: i64 = 10000000000000;

#[derive(Debug, Clone)]
pub struct Machine {
    a: Point,
    b: Point,
    prize: Point,
}

fn parse_input(input: &str) -> Result<Problem> {
    let re_button = Regex::new(r#"Button [AB]: X\+(\d+), Y\+(\d+)"#).unwrap();
    let re_prize = Regex::new(r#"Prize: X=(\d+), Y=(\d+)"#).unwrap();

    let lines: Vec<_> = input.lines().collect();
    let mut machines = Vec::new();
    for sp in lines.split(|l| l.is_empty()) {
        let cap_a = re_button.captures(sp[0]).ok_anyhow()?;
        let cap_b = re_button.captures(sp[1]).ok_anyhow()?;
        let cap_prize = re_prize.captures(sp[2]).ok_anyhow()?;
        let a = Point::new(cap_a[1].parse()?, cap_a[2].parse()?);
        let b = Point::new(cap_b[1].parse()?, cap_b[2].parse()?);
        let prize = Point::new(cap_prize[1].parse()?, cap_prize[2].parse()?);
        machines.push(Machine { a, b, prize });
    }

    Ok(Problem { machines })
}

// we should only really have one solution, so this is probably missing
// the mark
fn solve_brute(machine: &Machine) -> Option<i64> {
    let mut best_cost: Option<i64> = None;
    for a in 0..=100 {
        for b in 0..=100 {
            let loc_a = Point::new(a, a) * machine.a;
            let loc_b = Point::new(b, b) * machine.b;
            let loc = loc_a + loc_b;
            let cost = a * A_COST + b * B_COST;
            if loc == machine.prize {
                best_cost = best_cost.map(|bc| bc.min(cost)).or(Some(cost));
            }
        }
    }
    best_cost
}

// is just a simultaneous equation - provided we can find an
// integer solution, we're good.
fn solve_equation(machine: &Machine) -> Option<i64> {
    let x = machine.prize.x;
    let y = machine.prize.y;

    // x coeffs
    let c = machine.a.x;
    let d = machine.b.x;

    // y coeffs
    let e = machine.a.y;
    let f = machine.b.y;

    // solve for b
    let num_b = y * c - x * e;
    let den_b = c * f - d * e;
    if num_b % den_b != 0 {
        return None;
    }
    let b = num_b / den_b;

    // solve for a
    let num_a = x - b * d;
    if num_a % c != 0 {
        return None;
    }
    let a = num_a / c;

    Some(a * A_COST + b * B_COST)
}

fn part1(problem: &Problem, solver: impl Fn(&Machine) -> Option<i64>) -> Result<i64> {
    let mut total_cost = 0;
    for p in &problem.machines {
        if let Some(cost) = solver(p) {
            total_cost += cost;
        }
    }
    Ok(total_cost)
}

fn part2(problem: &Problem) -> Result<i64> {
    let mut total_cost = 0;
    for p in &problem.machines {
        let modified_machine = Machine {
            prize: p.prize + Point::new(PART2_OFFSET, PART2_OFFSET),
            ..*p
        };

        if let Some(cost) = solve_equation(&modified_machine) {
            total_cost += cost;
        }
    }
    Ok(total_cost)
}

fn main() -> anyhow::Result<()> {
    let text = common::read_file("input1.txt")?;
    let problem = parse_input(&text)?;

    let t = Instant::now();
    let count_part1 = part1(&problem, solve_brute)?;
    println!(
        "Part 1 (brute) result is {count_part1} (took {:?})",
        t.elapsed()
    );

    let t = Instant::now();
    let count_part1 = part1(&problem, solve_equation)?;
    println!(
        "Part 1 (equation) result is {count_part1} (took {:?})",
        t.elapsed()
    );

    let t = Instant::now();
    let count_part2 = part2(&problem)?;
    println!("Part 2 result is {count_part2} (took {:?})", t.elapsed());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
        Button A: X+94, Y+34
        Button B: X+22, Y+67
        Prize: X=8400, Y=5400

        Button A: X+26, Y+66
        Button B: X+67, Y+21
        Prize: X=12748, Y=12176

        Button A: X+17, Y+86
        Button B: X+84, Y+37
        Prize: X=7870, Y=6450

        Button A: X+69, Y+23
        Button B: X+27, Y+71
        Prize: X=18641, Y=10279
    "};

    #[test]
    fn test_parse_input() -> Result<()> {
        let problem = parse_input(EXAMPLE)?;
        println!("{:?}", problem);
        Ok(())
    }

    #[test]
    fn part1_correct_brute() -> Result<()> {
        let problem = parse_input(EXAMPLE)?;
        let count = part1(&problem, solve_brute)?;
        assert_eq!(count, 480);
        Ok(())
    }

    #[test]
    fn part1_correct_equation() -> Result<()> {
        let problem = parse_input(EXAMPLE)?;
        let count = part1(&problem, solve_equation)?;
        assert_eq!(count, 480);
        Ok(())
    }

    #[test]
    fn part2_has_solution() -> Result<()> {
        let problem = parse_input(EXAMPLE)?;
        let count = part2(&problem)?;
        assert!(count > 0);
        Ok(())
    }

    #[test]
    fn solver_second_machine_solves() {
        let problem = parse_input(EXAMPLE).unwrap();
        let machine = &problem.machines[1];
        let machine = Machine {
            prize: machine.prize + Point::new(PART2_OFFSET, PART2_OFFSET),
            ..*machine
        };
        let cost = solve_equation(&machine);
        assert!(cost.is_some());
    }
}
