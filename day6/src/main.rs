use anyhow::bail;
use common::OptionAnyhow;
use nalgebra::DMatrix;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Block {
    Empty,
    Wall,
    Guard,
}

type Map = DMatrix<Block>;

#[derive(Debug, Clone)]
pub struct Problem {
    map: Map,
}

fn parse_input(input: &str) -> anyhow::Result<Problem> {
    let lines: Vec<_> = input.lines().collect();

    let rows = lines.len();
    let cols = lines.iter().map(|l| l.chars().count()).max().ok_anyhow()?;

    let mut map = DMatrix::from_element(rows, cols, Block::Empty);
    for row in 0..rows {
        let line = lines[row];
        for (col, ch) in line.chars().enumerate() {
            let block_type = match ch {
                '.' => Block::Empty,
                '^' => Block::Guard,
                '#' => Block::Wall,
                _ => bail!("unexpected map character: {}", ch),
            };
            map[(row, col)] = block_type;
        }
    }

    Ok(Problem { map })
}

fn main() {
    println!("Hello, world!");
}
