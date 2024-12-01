use std::{collections::HashMap, iter};

fn main() -> anyhow::Result<()> {
    let text = common::read_file("input1.txt")?;

    let mut left_list = vec![];
    let mut right_list = vec![];

    for l in text.lines() {
        let mut fields = l.split_whitespace();
        let left: i32 = fields.next().unwrap().parse().unwrap();
        let right: i32 = fields.next().unwrap().parse().unwrap();
        left_list.push(left);
        right_list.push(right);
    }

    // part 1 - could just calculate the difference without the sort

    left_list.sort();
    right_list.sort();

    let mut total_difference = 0;
    for (l, r) in iter::zip(&left_list, &right_list) {
        total_difference += (r - l).abs();
    }

    println!("part 1: total difference {}", total_difference);

    // part 2 - get counts/freq of numbers on right side first...

    let mut right_counts = HashMap::new();
    for r in right_list {
        let entry = right_counts.entry(r).or_insert(0);
        *entry += 1;
    }

    let mut similarity_score = 0;
    for l in left_list {
        let right_count = right_counts.get(&l).copied().unwrap_or_default();
        similarity_score += l * right_count;
    }

    println!("part 2: similarity_score {}", similarity_score);

    Ok(())
}
