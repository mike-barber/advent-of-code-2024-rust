fn main() -> anyhow::Result<()> {
    let text = common::read_file("input1.txt")?;
    let input: Vec<_> = text
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect();

    let safe_count = input
        .iter()
        .filter(|report| safe_part_1(report.as_slice()))
        .count();

    println!("Part 1 safe count: {}", safe_count);

    Ok(())
}

fn safe_part_1(report: &[i32]) -> bool {
    let diffs = || report.windows(2).map(|v| v[1] - v[0]);

    let monotonic = diffs().all(|d| d < 0) || diffs().all(|d| d > 0);
    let magnitudes = diffs().all(|d| d.abs() >= 1 && d.abs() <= 3);

    monotonic && magnitudes
}
