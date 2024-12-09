use std::time::Instant;

use anyhow::Result;
use common::OptionAnyhow;

#[derive(Debug, Clone)]
struct Record {
    id: i32,
    len: i32,
    free_after: i32,
}

#[derive(Debug, Clone)]
pub struct Problem {
    files: Vec<Record>,
}
impl Problem {
    fn total_length(&self) -> i32 {
        self.files.iter().map(|r| r.len + r.free_after).sum()
    }
}

fn parse_input(input: &str) -> Result<Problem> {
    let mut files = Vec::new();

    let mut rem = input.trim();
    let mut id = 0;
    while rem.len() > 0 {
        let (len, r) = rem.split_at(1);
        let (free_after, r) = if r.len() > 0 { r.split_at(1) } else { ("0", r) };
        let len = len.parse()?;
        let free_after = free_after.parse()?;
        let record = Record {
            id,
            len,
            free_after,
        };
        files.push(record);
        id += 1;
        rem = r;
    }
    Ok(Problem { files })
}

fn create_disk(files: &[Record]) -> Vec<Option<i32>> {
    let mut disk: Vec<Option<i32>> = Vec::new();
    for record in files.iter() {
        for _ in 0..record.len {
            disk.push(Some(record.id));
        }
        for _ in 0..record.free_after {
            disk.push(None);
        }
    }
    disk
}

fn print_disk_map(files: &[Record]) {
    let disk = create_disk(&files);
    let mut disk_map = String::new();
    for x in disk.iter() {
        match x {
            Some(v) => disk_map.push_str(&v.to_string()),
            None => disk_map.push('.'),
        }
    }
    println!("{}", disk_map);
}

fn part1(problem: &Problem) -> Result<usize> {
    println!("total length {}", problem.total_length());

    let mut disk = create_disk(&problem.files);

    // let mut left = 0;
    // let mut right = disk.len() -1;
    loop {
        let left = disk.iter().position(|x| x.is_none()).ok_anyhow()?;
        let right = disk
            .iter()
            .enumerate()
            .rev()
            .find(|(_, x)| x.is_some())
            .ok_anyhow()?
            .0;
        if left < right {
            disk.swap(left, right);
        } else {
            break;
        }
    }

    let sum: usize = disk
        .iter()
        .enumerate()
        .filter_map(|(i, x)| match x {
            Some(x) => Some(*x as usize * i),
            None => None,
        })
        .sum();

    Ok(sum)
}

fn part2(problem: &Problem) -> Result<usize> {
    let mut files = problem.files.clone();

    let mut id = files.iter().map(|f| f.id).max().ok_anyhow()?;
    loop {
        //print_disk_map(&files);
        println!("id {}", id);
        let right = files.iter().position(|f| f.id == id).ok_anyhow()?;
        let right_prior = right - 1;
        let right_len = files[right].len;
        let insert_after = files
            .iter()
            .enumerate()
            .find(|(_, r)| r.free_after >= right_len)
            .map(|(i, _)| i);

        if let Some(i) = insert_after {
            if i < right {
                let new_left_free = files[i].free_after - right_len;
                let new_gap_free = right_len + files[right].free_after;
                files[right_prior].free_after += new_gap_free;
                let moved = files.remove(right);
                files.insert(i + 1, moved);
                files[i].free_after = 0;
                files[i + 1].free_after = new_left_free;
            }
        }

        if id > 1 {
            id -= 1;
        } else {
            break;
        }
    }

    let disk = create_disk(&files);
    let mut sum: usize = 0;
    for i in 0..disk.len() {
        if let Some(id) = disk[i] {
            sum = sum.checked_add(i as usize * id as usize).unwrap();
        }
    }

    Ok(sum)
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

    const EXAMPLE: &str = "2333133121414131402";

    #[test]
    fn test_parse_input() -> Result<()> {
        let problem = parse_input(EXAMPLE)?;
        println!("{:?}", problem);
        println!("total length {}", problem.total_length());
        Ok(())
    }

    #[test]
    fn part1_correct() -> Result<()> {
        let problem = parse_input(EXAMPLE)?;
        let count = part1(&problem)?;
        assert_eq!(count, 1928);
        Ok(())
    }

    #[test]
    fn part2_correct() -> Result<()> {
        let problem = parse_input(EXAMPLE)?;
        let count = part2(&problem)?;
        assert_eq!(count, 2858);
        Ok(())
    }
}
