use std::{ops::Range, time::Instant};

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
    while !rem.is_empty() {
        let (len, r) = rem.split_at(1);
        let (free_after, r) = if !r.is_empty() {
            r.split_at(1)
        } else {
            ("0", r)
        };
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

#[allow(dead_code)]
fn disk_map(disk: &[Option<i32>]) -> String {
    let mut disk_map = String::new();
    for x in disk.iter() {
        match x {
            Some(v) => {
                let print_num = v % 10;
                disk_map.push_str(&print_num.to_string());
            }
            None => disk_map.push('.'),
        }
    }
    disk_map
}
#[allow(dead_code)]
fn print_disk_map(disk: &[Option<i32>]) {
    println!("{}", disk_map(disk));
}

fn checksum_disk(disk: &[Option<i32>]) -> usize {
    let mut sum: usize = 0;
    for (i, id) in disk.iter().enumerate() {
        if let Some(id) = id {
            sum = sum.checked_add(i * *id as usize).unwrap();
        }
    }
    sum
}

fn part1(problem: &Problem) -> Result<usize> {
    println!("total length {}", problem.total_length());

    let mut disk = create_disk(&problem.files);

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

    Ok(checksum_disk(&disk))
}

/// Brute-force, copy-stuff-around approach that works
fn part2_brute(problem: &Problem) -> Result<usize> {
    fn find_id(disk: &[Option<i32>], id: i32) -> Option<Range<usize>> {
        if let Some(start) = disk.iter().position(|x| x == &Some(id)) {
            let end = disk[start..].iter().take_while(|x| *x == &Some(id)).count();
            let end = end + start;
            Some(start..end)
        } else {
            None
        }
    }

    fn find_space(disk: &[Option<i32>], required_len: usize) -> Option<usize> {
        disk.windows(required_len)
            .position(|w| w.iter().all(|x| x.is_none()))
    }

    let mut disk = create_disk(&problem.files);
    //print_disk_map(&disk);

    let max_id = problem.files.last().ok_anyhow()?.id;
    for id in (1..=max_id).rev() {
        // find the file we are considering moving
        let range_id = find_id(&disk, id).ok_anyhow()?;

        // find a potential location to the left of it
        let search_space = &disk[0..range_id.start];
        let required_len = range_id.clone().count();
        assert_eq!(required_len, problem.files[id as usize].len as usize);
        if let Some(dest) = find_space(search_space, required_len) {
            // move elements
            disk.copy_within(range_id.clone(), dest);
            // "delete" old
            disk[range_id.clone()].fill(None);
        }
    }

    //print_disk_map(&disk);
    Ok(checksum_disk(&disk))
}

/// This works, and is much more efficient, but required me to do the brute force
/// approach first in order to debug it. It passed the tests fine. Although a more
/// extensive set of my own unit tests would have revealed the problem.
fn part2_smarter(problem: &Problem) -> Result<usize> {
    let mut files = problem.files.clone();
    let initial_disk = create_disk(&files);

    let max_id = files.last().ok_anyhow()?.id;
    for id in (2..=max_id).rev() {
        let cur = files.iter().position(|f| f.id == id).ok_anyhow()?;
        let cur_prior = cur - 1;
        let required_len = files[cur].len;
        let dest_prior = files
            .iter()
            .enumerate()
            .find(|(_, r)| r.free_after >= required_len)
            .map(|(i, _)| i);

        match dest_prior {
            // we can move the file left into any space where it fits, including
            // the free space after the node immediately to the left of it.
            Some(dest_prior) if dest_prior <= cur_prior => {
                // existing location - give the space taken and space free to the prior node
                files[cur_prior].free_after =
                    files[cur_prior].free_after + files[cur].len + files[cur].free_after;

                // for the node we're moving, the free space to the right is the remaning space from dest_right
                files[cur].free_after = files[dest_prior].free_after - files[cur].len;

                // destination location - remove the space on the right of the destination node completely,
                // since we're placing the node directly to the right of it
                files[dest_prior].free_after = 0;

                // finally, move the file to the destination location,
                // inserting it to the right of the destination node
                let file = files.remove(cur);
                files.insert(dest_prior + 1, file);
            }
            _ => {}
        }
    }

    let disk = create_disk(&files);
    let sum = checksum_disk(&disk);
    assert_eq!(disk.len(), initial_disk.len());

    Ok(sum)
}

fn main() -> anyhow::Result<()> {
    let text = common::read_file("input1.txt")?;
    let problem = parse_input(&text)?;

    let t = Instant::now();
    let count_part1 = part1(&problem)?;
    println!("Part 1 result is {count_part1} (took {:?})", t.elapsed());

    let t = Instant::now();
    let count_part2 = part2_brute(&problem)?;
    println!(
        "Part 2 (brute) result is {count_part2} (took {:?})",
        t.elapsed()
    );

    let t = Instant::now();
    let count_part2 = part2_smarter(&problem)?;
    println!(
        "Part 2 (smart) result is {count_part2} (took {:?})",
        t.elapsed()
    );

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
    fn part2_smarter_correct() -> Result<()> {
        let problem = parse_input(EXAMPLE)?;
        let count = part2_smarter(&problem)?;
        assert_eq!(count, 2858);
        Ok(())
    }

    #[test]
    fn part2_brute_correct() -> Result<()> {
        let problem = parse_input(EXAMPLE)?;
        let count = part2_brute(&problem)?;
        assert_eq!(count, 2858);
        Ok(())
    }
}
