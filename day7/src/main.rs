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

mod brute {
    use crate::{concatenate, Equation, Op, OpsVec, Problem};
    use anyhow::Result;

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
            };
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

    pub fn part1(problem: &Problem) -> Result<i64> {
        let available_ops = [Op::Add, Op::Multiply];
        let mut sum = 0;
        for eq in problem.equations.iter() {
            if part1_solve(eq, &[], &available_ops) {
                sum += eq.test_value;
            }
        }
        Ok(sum)
    }

    pub fn part2(problem: &Problem) -> Result<i64> {
        let available_ops = [Op::Add, Op::Multiply, Op::Concatenate];
        let mut sum = 0;
        for eq in problem.equations.iter() {
            if part1_solve(eq, &[], &available_ops) {
                sum += eq.test_value;
            }
        }
        Ok(sum)
    }
}

mod smart {
    use crate::{concatenate, Op, Problem};
    use anyhow::Result;

    pub fn solve(
        test_case: i64,
        current_val: i64,
        remaining_numbers: &[i64],
        available_ops: &[Op],
    ) -> bool {
        // terminal case
        if remaining_numbers.is_empty() {
            return test_case == current_val;
        }

        // early break - numbers only increase
        if current_val > test_case {
            return false;
        }

        // DFS
        for op in available_ops.iter().copied() {
            let (&a, next_remaining) = remaining_numbers.split_first().unwrap();
            let next_val = match op {
                Op::Add => current_val.checked_add(a),
                Op::Multiply => current_val.checked_mul(a),
                Op::Concatenate => concatenate(current_val, a),
            };

            match next_val {
                Some(v) => {
                    if solve(test_case, v, next_remaining, available_ops) {
                        return true;
                    }
                }
                None => return false,
            }
        }
        false
    }

    pub fn part1(problem: &Problem) -> Result<i64> {
        let available_ops = [Op::Add, Op::Multiply];
        let mut sum = 0;
        for eq in problem.equations.iter() {
            let (init, remaining) = eq.numbers.split_first().unwrap();
            if solve(eq.test_value, *init, remaining, &available_ops) {
                sum += eq.test_value;
            }
        }
        Ok(sum)
    }

    pub fn part2(problem: &Problem) -> Result<i64> {
        let available_ops = [Op::Add, Op::Multiply, Op::Concatenate];
        let mut sum = 0;
        for eq in problem.equations.iter() {
            let (init, remaining) = eq.numbers.split_first().unwrap();
            if solve(eq.test_value, *init, remaining, &available_ops) {
                sum += eq.test_value;
            }
        }
        Ok(sum)
    }
}

fn main() -> anyhow::Result<()> {
    let text = common::read_file("input1.txt")?;

    let problem = parse_input(&text)?;

    println!("Initial solution - brute force");

    let t1 = Instant::now();
    let count_part1 = brute::part1(&problem)?;
    println!(
        "Brute: Part 1 count is {count_part1} (elapsed {:?})",
        t1.elapsed()
    );

    let t2 = Instant::now();
    let count_part2 = brute::part2(&problem)?;
    println!(
        "Brute: Part 2 count is {count_part2} (elapsed {:?})",
        t2.elapsed()
    );

    println!();
    println!("Smarter solution - additional early breakout and incremental calculation");

    let t1 = Instant::now();
    let count_part1 = smart::part1(&problem)?;
    println!(
        "Smart: Part 1 count is {count_part1} (elapsed {:?})",
        t1.elapsed()
    );

    let t2 = Instant::now();
    let count_part2 = smart::part2(&problem)?;
    println!(
        "Smart: Part 2 count is {count_part2} (elapsed {:?})",
        t2.elapsed()
    );

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
    fn part1_correct_brute() {
        let problem = parse_input(EXAMPLE).unwrap();
        let count = brute::part1(&problem).unwrap();
        assert_eq!(count, 3749);
    }

    #[test]
    fn part2_correct_brute() {
        let problem = parse_input(EXAMPLE).unwrap();
        let count = brute::part2(&problem).unwrap();
        assert_eq!(count, 11387);
    }

    #[test]
    fn part1_correct_smart() {
        let problem = parse_input(EXAMPLE).unwrap();
        let count = smart::part1(&problem).unwrap();
        assert_eq!(count, 3749);
    }

    #[test]
    fn part2_correct_smart() {
        let problem = parse_input(EXAMPLE).unwrap();
        let count = smart::part2(&problem).unwrap();
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
