use anyhow::Result;
use arrayvec::ArrayVec;
use common::OptionAnyhow;
use itertools::Itertools;
use std::{ptr::eq, time::Instant};

#[derive(Debug, Clone)]
struct Equation {
    test_value: i64,
    numbers: Vec<i64>,
}

#[derive(Debug, Clone)]
struct Problem {
    equations: Vec<Equation>,
}

#[derive(Debug, Clone, Copy)]
enum Op {
    Add,
    Multiply,
}

type OpsVec = ArrayVec<Op, 16>;

fn parse_input(input: &str) -> Result<Problem> {
    let mut equations = Vec::new();
    for l in input.lines() {
        let (test, rest) = l.split_once(':').ok_anyhow()?;
        let test_value = test.parse()?;
        let numbers = rest.split_whitespace().map(|n| n.parse()).try_collect()?;
        equations.push(Equation {
            test_value,
            numbers,
        });
    }
    Ok(Problem { equations })
}

fn evaluate_left_right(values: &[i64], operators: &[Op]) -> i64 {
    let mut vit = values.iter();
    let mut v = *vit.next().unwrap();

    for (a, op) in std::iter::zip(vit, operators.iter()) {
        v = match *op {
            Op::Add => v + a,
            Op::Multiply => v * a,
        }
    }

    v
}

fn part1_solve(equation: &Equation, operators: &[Op]) -> bool {
    // terminal case
    if operators.len() == equation.numbers.len() - 1 {
        let evaluated = evaluate_left_right(equation.numbers.as_slice(), operators);
        return evaluated == equation.test_value;
    }

    // DFS
    for op in [Op::Add, Op::Multiply] {
        let mut ops: OpsVec = operators.iter().copied().collect();
        ops.push(op);

        if part1_solve(equation, ops.as_slice()) {
            return true;
        }
    }

    return false;
}

fn part1(problem: &Problem) -> Result<i64> {
    let mut sum = 0;
    for eq in problem.equations.iter() {
        if part1_solve(eq, &[]) {
            sum += eq.test_value;
        }
    }
    Ok(sum)
}

fn part2(problem: &Problem) -> Result<i64> {
    todo!()
}

fn main() -> anyhow::Result<()> {
    let text = common::read_file("input1.txt")?;

    let problem = parse_input(&text)?;

    let t1 = Instant::now();
    let count_part1 = part1(&problem)?;
    println!("Part 1 count is {count_part1} (elapsed {:?})", t1.elapsed());

    let t2 = Instant::now();
    let count_part2 = part2(&problem)?;
    println!("Part 2 count is {count_part2} (elapsed {:?})", t2.elapsed());

    Ok(())
}

mod tests {
    use super::*;
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
        190: 10 19
        3267: 81 40 27
        83: 17 5
        156: 15 6
        7290: 6 8 6 15
        161011: 16 10 13
        192: 17 8 14
        21037: 9 7 18 13
        292: 11 6 16 20
    "};

    #[test]
    fn test_parse_input() {
        let _problem = parse_input(EXAMPLE).unwrap();
    }

    #[test]
    fn part1_correct() {
        let problem = parse_input(EXAMPLE).unwrap();
        let count = part1(&problem).unwrap();
        assert_eq!(count, 3749);
    }

    #[test]
    fn part2_correct() {
        let problem = parse_input(EXAMPLE).unwrap();
        let count = part2(&problem).unwrap();
        assert_eq!(count, 0);
    }
}
