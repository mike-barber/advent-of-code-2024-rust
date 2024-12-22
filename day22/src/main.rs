use std::time::Instant;

use anyhow::Result;
use common::OptionAnyhow;

#[derive(Debug, Clone)]
pub struct Problem {
    initial_numbers: Vec<i64>,
}

fn parse_input(input: &str) -> Result<Problem> {
    let initial_numbers = input
        .lines()
        .map(str::parse)
        .collect::<Result<Vec<i64>, _>>()?;
    Ok(Problem { initial_numbers })
}

fn next(n: i64) -> i64 {
    let n = ((n * 64) ^ n) % 16777216;
    let n = ((n / 32) ^ n) % 16777216;
    let n = ((n * 2048) ^ n) % 16777216;
    n
}

fn iterate(init: i64) -> impl Iterator<Item = i64> {
    std::iter::successors(Some(init), |n| Some(next(*n)))
}

fn part1(problem: &Problem) -> Result<i64> {
    let mut total = 0;
    for init in &problem.initial_numbers {
        let nth = iterate(*init).nth(2000).ok_anyhow()?;
        println!("{init} {nth}");
        total += nth;
    }

    Ok(total)
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

    const EXAMPLE: &str = indoc! {"
        1
        10
        100
        2024
    "};

    #[test]
    fn test_parse_input() -> Result<()> {
        let problem = parse_input(EXAMPLE)?;
        println!("{:?}", problem);
        Ok(())
    }

    #[test]
    fn iterate_correct() {
        let init = 123;
        let first10: Vec<_> = iterate(init).skip(1).take(10).collect();
        assert_eq!(
            first10,
            [
                15887950, 16495136, 527345, 704524, 1553684, 12683156, 11100544, 12249484, 7753432,
                5908254
            ]
        );
    }

    #[test]
    fn part1_correct() -> Result<()> {
        let problem = parse_input(EXAMPLE)?;
        let count = part1(&problem)?;
        assert_eq!(count, 37327623);
        Ok(())
    }

    #[test]
    fn part2_correct() -> Result<()> {
        let problem = parse_input(EXAMPLE)?;
        let count = part2(&problem)?;
        assert_eq!(count, 2);
        Ok(())
    }
}
