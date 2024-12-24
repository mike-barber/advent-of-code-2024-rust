use std::time::Instant;

use anyhow::{bail, Result};
use common::OptionAnyhow;
use fxhash::{FxHashMap, FxHashSet};

type Value = Option<bool>;

#[derive(Debug, Clone)]
enum Operation {
    And,
    Or,
    Xor,
}
impl Operation {
    fn apply(&self, a: bool, b: bool) -> bool {
        match self {
            Operation::And => a && b,
            Operation::Or => a || b,
            Operation::Xor => a ^ b,
        }
    }
}

type Calculation<'a> = (Operation, &'a str, &'a str);

#[derive(Debug, Clone)]
pub struct Problem<'a> {
    initial_values: FxHashMap<&'a str, Value>,
    calculated: FxHashMap<&'a str, Calculation<'a>>,
}

fn parse_input(input: &str) -> Result<Problem> {
    let mut initial_values = FxHashMap::default();

    let mut lines = input.lines();
    for line in lines.by_ref() {
        if line.is_empty() {
            break;
        }

        let (id, val) = line.split_once(": ").ok_anyhow()?;
        let val = match val {
            "0" => Some(false),
            "1" => Some(true),
            _ => bail!("Unexpected value {val}"),
        };
        initial_values.insert(id, val);
    }

    let mut calculated = FxHashMap::default();
    for line in lines.by_ref() {
        let mut fields = line.split_whitespace();
        let ida = fields.next().ok_anyhow()?;
        let op = fields.next().ok_anyhow()?;
        let idb = fields.next().ok_anyhow()?;

        // skip arrow
        fields.next().ok_anyhow()?;

        let id = fields.next().ok_anyhow()?;

        let op = match op {
            "AND" => Operation::And,
            "OR" => Operation::Or,
            "XOR" => Operation::Xor,
            _ => bail!("Unrecognized operation {op}"),
        };

        calculated.insert(id, (op, ida, idb));
    }

    Ok(Problem {
        initial_values,
        calculated,
    })
}

fn calculate<'a>(
    mut registers: FxHashMap<&'a str, Value>,
    mut remaining_calculations: FxHashMap<&'a str, Calculation>,
) -> Result<(u64, FxHashMap<&'a str, Value>)> {
    while !remaining_calculations.is_empty() {
        remaining_calculations.retain(|id, calc| {
            let (op, ida, idb) = calc;
            let va = registers.get(ida).copied().flatten();
            let vb = registers.get(idb).copied().flatten();
            match (va, vb) {
                (Some(a), Some(b)) => {
                    let c = op.apply(a, b);
                    registers.insert(id, Some(c));

                    // completed this calculation - do not retain
                    false
                }
                _ => true, // retain for next iteration
            }
        });
    }

    // collect z values
    let mut total = 0;
    for i in 0.. {
        let id = format!("z{i:02}");
        let v = registers.get(id.as_str()).copied().flatten();

        if v.is_none() {
            break;
        }
        let v = v.unwrap();
        let v = v as u64;
        total += v << i;
    }

    Ok((total, registers))
}

fn part1(problem: &Problem) -> Result<u64> {
    let registers = problem.initial_values.clone();
    let remaining_calculations = problem.calculated.clone();
    let (result, _) = calculate(registers, remaining_calculations)?;
    Ok(result)
}

fn precendents_for<'a>(problem: &'a Problem, id: &'a str, found_ids: &mut FxHashSet<&'a str>) {
    if let Some(calc) = problem.calculated.get(id) {
        let (_, a, b) = *calc;
        if found_ids.insert(a) {
            precendents_for(problem, a, found_ids);
        }
        if found_ids.insert(b) {
            precendents_for(problem, b, found_ids);
        }
    }
}

fn get_id(label: char, index: i32) -> String {
    format!("{label}{index:02}")
}

fn get_idz(index: i32) -> String {
    get_id('z', index)
}

fn get_idx(index: i32) -> String {
    get_id('x', index)
}

fn get_idy(index: i32) -> String {
    get_id('y', index)
}

fn swap<'a>(
    mut calcs: FxHashMap<&'a str, Calculation<'a>>,
    a: &'a str,
    b: &'a str,
) -> FxHashMap<&'a str, Calculation<'a>> {
    let temp = (Operation::And, "", "");
    let calc_a = calcs.entry(a).or_insert(temp.clone()).clone();
    let calc_b = calcs.entry(b).or_insert(temp.clone()).clone();

    *calcs.entry(a).or_insert(temp.clone()) = calc_b;
    *calcs.entry(b).or_insert(temp.clone()) = calc_a;

    calcs
}

fn part2(problem: &Problem) -> Result<String> {
    let Problem {
        mut calculated,
        initial_values,
    } = problem.clone();

    let swaps = [
        ("z17", "cmv"), // swap 1 - this fixes bit 17
        ("z23", "rmj"), // swap 2 - this fixes bit 22
        ("z30", "rdg"), // swap 3 - this fixes bit 30
        ("btb", "mwp"), // swap 4 - this fixes bit 38
    ];

    for (a, b) in swaps {
        calculated = swap(calculated, a, b);
    }

    let problem = Problem {
        calculated,
        initial_values,
    };
    let errors = tests(&problem)?;
    println!("remaining errors: {errors}");

    let mut swaps_flat: Vec<_> = swaps.iter().flat_map(|s| [s.0, s.1]).collect();
    swaps_flat.sort();

    Ok(swaps_flat.join(","))
}

fn tests(problem: &Problem) -> Result<usize> {
    let mut error_count = 0;

    // find largest bit
    let msb = (0..63)
        .filter(|b| problem.calculated.contains_key(get_idz(*b).as_str()))
        .last()
        .unwrap();
    println!("msb {msb}");

    // trace precendents for each bit
    let idzs: Vec<String> = (0..=msb).map(get_idz).collect();
    let mut prev_preceding = FxHashSet::default();
    for i in 0..=msb {
        let id = &idzs[i as usize];

        let mut preceding = FxHashSet::default();
        precendents_for(problem, id.as_str(), &mut preceding);

        let added = preceding.difference(&prev_preceding);
        println!("{id} depends on added {added:?}");

        // checks
        for u in 0..=i {
            if i == msb {
                continue;
            }
            let idx = get_idx(u);
            let idy = get_idy(u);
            if !preceding.contains(idx.as_str()) {
                println!("{id} missing dependence on {idx}");
                error_count += 1;
            }
            if !preceding.contains(idy.as_str()) {
                println!("{id} missing dependence on {idy}");
                error_count += 1;
            }
        }

        for u in i + 1..=msb {
            let idx = get_idx(u);
            let idy = get_idy(u);
            if preceding.contains(idx.as_str()) {
                println!("{id} should not depend {idx}");
                error_count += 1;
            }
            if preceding.contains(idy.as_str()) {
                println!("{id} should not depend {idy}");
                error_count += 1;
            }
        }

        prev_preceding = preceding;
    }

    // bit tests
    let inputs = [(false, false), (false, true), (true, false), (true, true)];
    for i in 0..=(msb - 1) {
        for (x, y) in inputs {
            // setup registers
            let mut registers = problem.initial_values.clone();
            for j in 0..=(msb - 1) {
                *registers.get_mut(get_idx(j).as_str()).unwrap() = Some(false);
                *registers.get_mut(get_idy(j).as_str()).unwrap() = Some(false);
            }
            *registers.get_mut(get_idx(i).as_str()).unwrap() = Some(x);
            *registers.get_mut(get_idy(i).as_str()).unwrap() = Some(y);

            // expected z and carry bit
            let expected_z = (x ^ y) as u64;
            let expected_carry = (x && y) as u64;
            let expected_res = (expected_z << i) | (expected_carry << (i + 1));

            // run calculation and check the results
            let remaining_calculations = problem.calculated.clone();
            let (result, registers_post) = calculate(registers, remaining_calculations).unwrap();

            let expected_z = registers_post
                .get(get_idx(i).as_str())
                .copied()
                .flatten()
                .unwrap() as u64;
            let expected_carry = registers_post
                .get(get_idx(i).as_str())
                .copied()
                .flatten()
                .unwrap() as u64;

            if expected_res != result {
                let x = x as u64;
                let y = y as u64;
                println!("Unexpected result for bit {i} - input {x},{y} got {expected_z} carry {expected_carry}; totals got {result} expected {expected_res}");
                error_count += 1;
            }
        }
    }
    Ok(error_count)
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
    use indoc::indoc;

    #[test]
    fn test_parse_input() -> Result<()> {
        let problem = parse_input(EXAMPLE)?;
        println!("{:?}", problem);
        Ok(())
    }

    #[test]
    fn part1_small_correct() -> Result<()> {
        let problem = parse_input(SMALL_EXAMPLE)?;
        let count = part1(&problem)?;
        assert_eq!(count, 4);
        Ok(())
    }

    #[test]
    fn part1_correct() -> Result<()> {
        let problem = parse_input(EXAMPLE)?;
        let count = part1(&problem)?;
        assert_eq!(count, 2024);
        Ok(())
    }

    const SMALL_EXAMPLE: &str = indoc! {"
        x00: 1
        x01: 1
        x02: 1
        y00: 0
        y01: 1
        y02: 0

        x00 AND y00 -> z00
        x01 XOR y01 -> z01
        x02 OR y02 -> z02
    "};

    const EXAMPLE: &str = indoc! {"
        x00: 1
        x01: 0
        x02: 1
        x03: 1
        x04: 0
        y00: 1
        y01: 1
        y02: 1
        y03: 1
        y04: 1

        ntg XOR fgs -> mjb
        y02 OR x01 -> tnw
        kwq OR kpj -> z05
        x00 OR x03 -> fst
        tgd XOR rvg -> z01
        vdt OR tnw -> bfw
        bfw AND frj -> z10
        ffh OR nrd -> bqk
        y00 AND y03 -> djm
        y03 OR y00 -> psh
        bqk OR frj -> z08
        tnw OR fst -> frj
        gnj AND tgd -> z11
        bfw XOR mjb -> z00
        x03 OR x00 -> vdt
        gnj AND wpb -> z02
        x04 AND y00 -> kjc
        djm OR pbm -> qhw
        nrd AND vdt -> hwm
        kjc AND fst -> rvg
        y04 OR y02 -> fgs
        y01 AND x02 -> pbm
        ntg OR kjc -> kwq
        psh XOR fgs -> tgd
        qhw XOR tgd -> z09
        pbm OR djm -> kpj
        x03 XOR y03 -> ffh
        x00 XOR y04 -> ntg
        bfw OR bqk -> z06
        nrd XOR fgs -> wpb
        frj XOR qhw -> z04
        bqk OR frj -> z07
        y03 OR x01 -> nrd
        hwm AND bqk -> z03
        tgd XOR rvg -> z12
        tnw OR pbm -> gnj
    "};
}
