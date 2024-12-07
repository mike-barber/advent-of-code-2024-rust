use anyhow::Result;
use std::time::Instant;

#[derive(Debug, Clone)]
pub struct Problem {}

fn parse_input(input: &str) -> Result<Problem> {
    todo!()
}

fn part1(problem: &Problem) -> Result<usize> {
    todo!()
}

fn part2(problem: &Problem) -> Result<usize> {
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
        
    "};

    #[test]
    fn test_parse_input() {
        let problem = parse_input(EXAMPLE).unwrap();
        println!("{:?}", problem);
    }

    #[test]
    fn part1_correct() {
        let problem = parse_input(EXAMPLE).unwrap();
        let count = part1(&problem).unwrap();
        assert_eq!(count, 0);
    }

    #[test]
    fn part2_correct() {
        let problem = parse_input(EXAMPLE).unwrap();
        let count = part2(&problem).unwrap();
        assert_eq!(count, 0);
    }
}
