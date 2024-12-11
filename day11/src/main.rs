use std::{
    iter,
    ops::{Div, Rem},
    time::Instant,
};

use anyhow::Result;
use common::OptionAnyhow;
use dlv_list::VecList;
use rustc_hash::FxHashMap;

#[derive(Debug, Clone)]
pub struct Problem {
    stones: VecList<i64>,
}

fn parse_input(input: &str) -> Result<Problem> {
    let mut stones = VecList::new();
    for n in input.split_whitespace() {
        stones.push_back(n.parse()?);
    }
    Ok(Problem { stones })
}

fn try_split(n: i64) -> Option<(i64, i64)> {
    let order = n.ilog10() + 1;
    if order % 2 == 0 {
        let factor = iter::successors(Some(1), |a| Some(a * 10))
            .nth(order as usize / 2)
            .expect("factor");
        Some((n.div(factor), n.rem(factor)))
    } else {
        None
    }
}

fn iterate(stones: &VecList<i64>, iterations: usize) -> Result<usize> {
    let mut stones = stones.clone();
    for _ in 0..iterations {
        let mut ix = stones.front_index().ok_anyhow()?;
        loop {
            match stones.get(ix).copied().ok_anyhow()? {
                0 => *stones.get_mut(ix).ok_anyhow()? = 1,
                n => {
                    if let Some((a, b)) = try_split(n) {
                        stones.insert_before(ix, a);
                        *stones.get_mut(ix).ok_anyhow()? = b;
                    } else {
                        *stones.get_mut(ix).ok_anyhow()? = n.checked_mul(2024).ok_anyhow()?;
                    }
                }
            }
            if let Some(next) = stones.get_next_index(ix) {
                ix = next;
            } else {
                break;
            }
        }
    }
    let num_stones = stones.len();
    Ok(num_stones)
}

fn part1(problem: &Problem) -> Result<usize> {
    iterate(&problem.stones, 25)
}

fn iterate_recurse_count(n: i64, remaining_depth: usize) -> usize {
    if remaining_depth == 0 {
        return 1;
    }
    match n {
        0 => {
            let a = 1;
            iterate_recurse_count(a, remaining_depth - 1)
        }
        n => {
            if let Some((a, b)) = try_split(n) {
                let num_a = iterate_recurse_count(a, remaining_depth - 1);
                let num_b = iterate_recurse_count(b, remaining_depth - 1);
                num_a + num_b
            } else {
                let a = n.checked_mul(2024).expect("overflow");
                iterate_recurse_count(a, remaining_depth - 1)
            }
        }
    }
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
struct Key(i64, usize);
type Cache = FxHashMap<Key, usize>;

/// Recursive with memoization. Large values eventually split to smaller values, so
/// we don't need to try to memoize everything - just storing the small values is enough.
fn iterate_recurse_count_mem(n: i64, remaining_depth: usize, memory: &mut Cache) -> usize {
    // termination
    if remaining_depth == 0 {
        return 1;
    }

    // already-computed value
    if let Some(mem) = memory.get(&Key(n, remaining_depth)) {
        return *mem;
    }

    // otherwise iterate
    let count = match n {
        0 => {
            let a = 1;
            iterate_recurse_count_mem(a, remaining_depth - 1, memory)
        }
        n => {
            if let Some((a, b)) = try_split(n) {
                let num_a = iterate_recurse_count_mem(a, remaining_depth - 1, memory);
                let num_b = iterate_recurse_count_mem(b, remaining_depth - 1, memory);
                num_a + num_b
            } else {
                let a = n.checked_mul(2024).expect("overflow");
                iterate_recurse_count_mem(a, remaining_depth - 1, memory)
            }
        }
    };

    // store smaller values of n in the cache
    if n <= 1024 {
        memory.insert(Key(n, remaining_depth), count);
    }

    count
}

fn part2(problem: &Problem, iterations: usize) -> Result<usize> {
    // memory can be used across multiple calls
    let mut mem = Cache::default();
    let mut total = 0;
    for n in &problem.stones {
        total += iterate_recurse_count_mem(*n, iterations, &mut mem);
    }
    Ok(total)
}

fn main() -> anyhow::Result<()> {
    let text = common::read_file("input1.txt")?;
    let problem = parse_input(&text)?;

    let t1 = Instant::now();
    let count_part1 = part1(&problem)?;
    println!("Part 1 result is {count_part1} (took {:?})", t1.elapsed());

    // try iterate simple
    let t = Instant::now();
    let nn = iterate_recurse_count(0, 30);
    println!("{nn} in {:?}", t.elapsed());

    // try iterate memoized
    let t = Instant::now();
    let mut mem = Cache::default();
    let nn = iterate_recurse_count_mem(0, 30, &mut mem);
    println!("{nn} in {:?}", t.elapsed());

    // part 2 result
    let t2 = Instant::now();
    let count_part2 = part2(&problem, 75)?;
    println!("Part 2 result is {count_part2} (took {:?})", t2.elapsed());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
        125 17
    "};

    #[test]
    fn split_correct() {
        assert_eq!(try_split(1000).unwrap(), (10, 0));
        assert_eq!(try_split(10).unwrap(), (1, 0));
        assert_eq!(try_split(111222).unwrap(), (111, 222));
    }

    #[test]
    fn test_parse_input() -> Result<()> {
        let problem = parse_input(EXAMPLE)?;
        println!("{:?}", problem);
        Ok(())
    }

    #[test]
    fn part1_correct() -> Result<()> {
        let problem = parse_input(EXAMPLE)?;
        let count = part1(&problem)?;
        assert_eq!(count, 55312);
        Ok(())
    }

    #[test]
    fn part2_correct() -> Result<()> {
        let problem = parse_input(EXAMPLE)?;
        let count = part2(&problem, 25)?;
        assert_eq!(count, 55312);
        Ok(())
    }
}
