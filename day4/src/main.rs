//use regex::Regex;

#[derive(Debug, Copy, Clone)]
struct Dir(i32, i32);

#[derive(Debug, Clone)]
struct Problem {
    matrix: Vec<Vec<char>>,
    rows: usize,
    cols: usize,
}

impl Problem {
    fn characters(&self, x: usize, y: usize, dir: Dir, buf: &mut String) {
        buf.clear();

        let mut x = x as i32;
        let mut y = y as i32;

        while x >= 0 && x < self.cols as i32 && y >= 0 && y < self.rows as i32 {
            buf.push(self.matrix[y as usize][x as usize]);
            x += dir.0;
            y += dir.1;
        }
    }
}

fn main() -> anyhow::Result<()> {
    let text = common::read_file("input1.txt")?;

    let problem = parse(&text);

    let count_part1 = part1(&problem);
    println!("Part 1 count is {count_part1}");

    Ok(())
}

fn directions() -> Vec<Dir> {
    let mut dirs = vec![];
    for x in -1..=1 {
        for y in -1..=1 {
            if x == 0 && y == 0 {
                continue;
            }
            dirs.push(Dir(x, y));
        }
    }
    dirs
}

fn parse(input: &str) -> Problem {
    let matrix: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let rows = matrix.len();
    let cols = matrix[0].len();
    Problem { matrix, rows, cols }
}

fn part1(problem: &Problem) -> usize {
    let dirs = directions();
    //let re = Regex::new(r#"XMAS"#).unwrap();

    let mut buf = String::new();
    let mut count = 0;
    for dir in &dirs {
        for x in 0..problem.cols {
            for y in 0..problem.rows {
                problem.characters(x, y, *dir, &mut buf);
                let found = buf.starts_with("XMAS");
                // let found = re.find_iter(&buf).count();
                if found {
                    //println!("{x},{y} dir {dir:?} buf {buf}");
                    count += 1; 
                }
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use crate::*;

    const EXAMPLE_SIMPLE: &str = indoc::indoc! {"
    ..X...
    .SAMX.
    .A..A.
    XMAS.S
    .X....
    "};

    const EXAMPLE: &str = indoc::indoc! {"
        MMMSXXMASM
        MSAMXMSMSA
        AMXSXMAAMM
        MSAMASMSMX
        XMASAMXAMM
        XXAMMXXAMA
        SMSMSASXSS
        SAXAMASAAA
        MAMMMXMMMM
        MXMXAXMASX
    "};

    #[test]
    fn part1_basic_correct() {
        let problem = parse(EXAMPLE_SIMPLE);
        let count = part1(&problem);
        assert_eq!(count, 4);
    }

    #[test]
    fn part1_correct() {
        let problem = parse(EXAMPLE);
        let count = part1(&problem);
        assert_eq!(count, 18);
    }

    #[test]
    fn part2_correct() {}
}
