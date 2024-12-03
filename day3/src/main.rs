use regex::Regex;

fn main() -> anyhow::Result<()> {
    let text = common::read_file("input1.txt")?;

    let sum_part1 = part1(&text)?;
    println!("part 1 sum is {sum_part1}");

    let sum_part2 = part2(&text)?;
    println!("part 2 sum is {sum_part2}");

    Ok(())
}

fn part1(input: &str) -> anyhow::Result<i32> {
    let re = Regex::new(r#"mul\((\d+),(\d+)\)"#).unwrap();

    let mut sum = 0;
    for cap in re.captures_iter(input) {
        let (_, [l, r]) = cap.extract();
        let l: i32 = l.parse()?;
        let r: i32 = r.parse()?;
        sum += l * r;
    }

    Ok(sum)
}

fn part2(input: &str) -> anyhow::Result<i32> {
    let re = Regex::new(r#"mul\((\d+),(\d+)\)|don't|do"#).unwrap();

    let mut enabled = true;
    let mut sum = 0;
    for cap in re.captures_iter(input) {
        match &cap[0] {
            "do" => enabled = true,
            "don't" => enabled = false,
            _ => {
                if enabled {
                    let l: i32 = cap[1].parse()?;
                    let r: i32 = cap[2].parse()?;
                    sum += l * r;
                }
            }
        }
    }

    Ok(sum)
}

#[cfg(test)]
mod tests {
    use crate::*;

    const EXAMPLE: &str = indoc::indoc! {"
        xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
    "};

    #[test]
    fn part1_correct() {
        let sum = part1(EXAMPLE).unwrap();
        assert_eq!(sum, 161);
    }

    #[test]
    fn part2_correct() {
        let sum = part2(EXAMPLE).unwrap();
        assert_eq!(sum, 48);
    }
}
