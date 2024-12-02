fn main() -> anyhow::Result<()> {
    let text = common::read_file("input1.txt")?;
    let reports: Result<Vec<_>, _> = text
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|n| n.parse())
                .collect::<Result<Vec<i32>, _>>()
        })
        .collect();
    let reports = reports?;

    let safe_count_1 = reports
        .iter()
        .filter(|report| safe_part_1(report.as_slice()))
        .count();
    println!("Part 1 safe count: {}", safe_count_1);

    let safe_count_2 = reports
        .iter()
        .filter(|report| safe_part_2(report.as_slice()))
        .count();
    println!("Part 2 safe count: {}", safe_count_2);

    Ok(())
}

fn safe_part_1(report: &[i32]) -> bool {
    let diffs = || report.windows(2).map(|v| v[1] - v[0]);

    let monotonic = diffs().all(|d| d < 0) || diffs().all(|d| d > 0);
    let magnitudes = diffs().all(|d| (1..=3).contains(&d.abs()));

    monotonic && magnitudes
}

fn safe_part_2(report: &[i32]) -> bool {
    // original case
    if safe_part_1(report) {
        return true;
    }

    // brute force removing one element at a time
    for idx_removed in 0..report.len() {
        let mut dampened = report.to_vec();
        dampened.remove(idx_removed);

        if safe_part_1(&dampened) {
            return true;
        }
    }

    false
}
