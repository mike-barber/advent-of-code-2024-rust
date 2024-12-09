use std::{ops::RangeInclusive, time::Instant};

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

#[allow(dead_code)]
fn print_disk_map(disk: &[Option<i32>]) {
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
    println!("{}", disk_map);
}

fn checksum_disk(disk: &[Option<i32>]) -> usize {
    let mut sum: usize = 0;
    for i in 0..disk.len() {
        if let Some(id) = disk[i] {
            sum = sum.checked_add(i as usize * id as usize).unwrap();
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

fn part2(problem: &Problem) -> Result<usize> {
    let mut files = problem.files.clone();
    println!("{files:?}");
    let initial_disk = create_disk(&files);
    println!("initial length {}", initial_disk.len());
    print_disk_map(&initial_disk);

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
            Some(dest_prior) if dest_prior < cur_prior => {
                // if we found a location to move it to, do the space accounting first, then do the move
                println!(
                    "{}[{:?}] -> {}[{:?}]",
                    cur, files[cur], dest_prior, files[dest_prior]
                );

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
            _ => {
                println!("{}[{:?}] -- not moving --", cur, files[cur]);
            }
        }
    }

    let disk = create_disk(&files);
    let sum = checksum_disk(&disk);
    print_disk_map(&disk);

    println!("final length {}", disk.len());
    assert_eq!(disk.len(), initial_disk.len());

    Ok(sum)
}

fn part2_brute(problem: &Problem) -> Result<usize> {
    fn find_id(disk: &[Option<i32>], id: i32) -> Option<RangeInclusive<usize>> {
        if let Some(start) = disk.iter().position(|x| x == &Some(id)) {
            let end = disk[start..].iter().take_while(|x| *x == &Some(id)).count();
            let end = end + start - 1;
            Some(start..=end)
        } else {
            None
        }
    }

    fn find_space(disk: &[Option<i32>], required_len: usize) -> Option<usize> {
        disk.windows(required_len as usize)
            .position(|w| w.iter().all(|x| x.is_none()))
    }

    let mut disk = create_disk(&problem.files);
    print_disk_map(&disk);

    let max_id = problem.files.last().ok_anyhow()?.id;
    for id in (1..=max_id).rev() {
        // find the file we are considering moving
        let range_id = find_id(&disk, id).ok_anyhow()?;

        // find a potential location to the left of it
        let search_space = &disk[0..*range_id.start()];
        let required_len = range_id.clone().count();
        assert_eq!(required_len, problem.files[id as usize].len as usize);
        if let Some(dest) = find_space(search_space, required_len) {
            // move elements
            disk.copy_within(range_id.clone(), dest);
            // "delete" old
            disk[range_id.clone()].fill(None);
        }
    }

    print_disk_map(&disk);
    Ok(checksum_disk(&disk))
}

fn main() -> anyhow::Result<()> {
    let text = common::read_file("input1.txt")?;
    let problem = parse_input(&text)?;

    let t = Instant::now();
    let count_part1 = part1(&problem)?;
    println!("Part 1 result is {count_part1} (took {:?})", t.elapsed());

    let t = Instant::now();
    let count_part2 = part2(&problem)?;
    println!("Part 2 result is {count_part2} (took {:?})", t.elapsed());

    let t = Instant::now();
    let count_part2 = part2_brute(&problem)?;
    println!("Part 2 (brute) result is {count_part2} (took {:?})", t.elapsed());

    {
        let mut rem = text.trim();
        let mut total = 0;
        while !rem.is_empty() {
            let (first, r) = rem.split_at(1);
            rem = r;
            let num: i32 = first.parse()?;
            total += num;
        }
        println!("total {}", total);
    }

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

    #[test]
    fn part2_brute_correct() -> Result<()> {
        let problem = parse_input(EXAMPLE)?;
        let count = part2_brute(&problem)?;
        assert_eq!(count, 2858);
        Ok(())
    }
}
