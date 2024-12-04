#[derive(Debug, Copy, Clone)]
struct Dir(i32, i32);

#[derive(Debug, Clone)]
struct Problem {
    matrix: Vec<Vec<char>>,
    rows: usize,
    cols: usize,
}

impl Problem {
    fn characters(&self, x: usize, y: usize, max_length: usize, dir: Dir, buf: &mut String) {
        buf.clear();

        let mut x = x as i32;
        let mut y = y as i32;

        while x >= 0
            && x < self.cols as i32
            && y >= 0
            && y < self.rows as i32
            && buf.len() < max_length
        {
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

    let count_part2 = part2(&problem);
    println!("Part 2 count is {count_part2}");

    Ok(())
}

fn parse(input: &str) -> Problem {
    let matrix: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let rows = matrix.len();
    let cols = matrix[0].len();
    Problem { matrix, rows, cols }
}

// not optimal by any means, but it's small enough to work
fn part1(problem: &Problem) -> usize {
    let dirs = [Dir(0, 1), Dir(1, 0), Dir(1, 1), Dir(1, -1)];

    let mut buf = String::new();
    let mut count = 0;
    for dir in &dirs {
        for x in 0..problem.cols {
            for y in 0..problem.rows {
                problem.characters(x, y, 4, *dir, &mut buf);
                let found = buf == "XMAS" || buf == "SAMX";
                if found {
                    count += 1;
                }
            }
        }
    }
    count
}

// this could be neater; pity the Direction abstraction wasn't useful here
fn part2(problem: &Problem) -> usize {
    let mut count = 0;
    for x in 1..problem.cols - 1 {
        for y in 1..problem.rows - 1 {
            if problem.matrix[y][x] == 'A' {
                let tl = problem.matrix[y - 1][x - 1];
                let tr = problem.matrix[y - 1][x + 1];
                let bl = problem.matrix[y + 1][x - 1];
                let br = problem.matrix[y + 1][x + 1];

                let matches = |a, b| matches!((a, b), ('M', 'S') | ('S', 'M'));
                let diag_down = matches(tl, br);
                let diag_up = matches(bl, tr);

                if diag_up && diag_down {
                    count += 1
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
    fn part2_correct() {
        let problem = parse(EXAMPLE);
        let count = part2(&problem);
        assert_eq!(count, 9);
    }
}
