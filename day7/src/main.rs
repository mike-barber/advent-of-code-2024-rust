use anyhow::Result;
use arrayvec::ArrayVec;
use common::OptionAnyhow;
use itertools::Itertools;
use std::time::Instant;

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
    Concatenate,
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

fn concatenate(a: i64, b: i64) -> Option<i64> {
    let mut btemp = b;
    let mut a = a * 10;
    while btemp.abs() >= 10 {
        btemp /= 10;
        a = a.checked_mul(10)?;
    }
    a.checked_add(b)
}

fn evaluate_left_right(values: &[i64], operators: &[Op]) -> Option<i64> {
    let mut vit = values.iter();
    let mut v = *vit.next().unwrap();

    for (a, op) in std::iter::zip(vit, operators.iter()) {
        v = match *op {
            Op::Add => v.checked_add(*a)?,
            Op::Multiply => v.checked_mul(*a)?,
            Op::Concatenate => concatenate(v, *a)?,
        }
    }

    Some(v)
}

fn part1_solve(equation: &Equation, operators: &[Op], available_ops: &[Op]) -> bool {
    // terminal case
    if operators.len() == equation.numbers.len() - 1 {
        return match evaluate_left_right(equation.numbers.as_slice(), operators) {
            Some(v) => v == equation.test_value,
            None => false,
        }
    }

    // DFS
    for op in available_ops.iter().copied() {
        let mut ops: OpsVec = operators.iter().copied().collect();
        ops.push(op);

        if part1_solve(equation, ops.as_slice(), available_ops) {
            return true;
        }
    }

    false
}

fn part1(problem: &Problem) -> Result<i64> {
    let available_ops = [Op::Add, Op::Multiply];
    let mut sum = 0;
    for eq in problem.equations.iter() {
        if part1_solve(eq, &[], &available_ops) {
            sum += eq.test_value;
        }
    }
    Ok(sum)
}

fn part2(problem: &Problem) -> Result<i64> {
    let available_ops = [Op::Add, Op::Multiply, Op::Concatenate];
    let mut sum = 0;
    for eq in problem.equations.iter() {
        if part1_solve(eq, &[], &available_ops) {
            sum += eq.test_value;
        }
    }
    Ok(sum)
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

#[cfg(test)]
mod tests {
    use crate::*;
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
        assert_eq!(count, 11387);
    }

    #[test]
    fn concatenate_correct() {
        assert_eq!(concatenate(1, 1).unwrap(), 11);
        assert_eq!(concatenate(1, 0).unwrap(), 10);
        assert_eq!(concatenate(0, 1).unwrap(), 1);
        assert_eq!(concatenate(15, 6).unwrap(), 156);
    }
}
