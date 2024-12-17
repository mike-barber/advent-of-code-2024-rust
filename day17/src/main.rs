use std::{path::Component, time::Instant};

use anyhow::{bail, Result};
use common::OptionAnyhow;
use indoc::indoc;
use itertools::Itertools;

const INPUT: &str = indoc! {"
    Register A: 27575648
    Register B: 0
    Register C: 0

    Program: 2,4,1,2,7,5,4,1,1,3,5,5,0,3,3,0
"};

#[derive(Debug, Clone)]
pub struct Computer {
    reg_a: i64,
    reg_b: i64,
    reg_c: i64,
    program: Vec<u8>,
    ip: usize,
    output: Vec<u8>,
}
impl Computer {
    fn new(a: i64, b: i64, c: i64, program: Vec<u8>) -> Computer {
        Computer {
            reg_a: a,
            reg_b: b,
            reg_c: c,
            program,
            ip: 0,
            output: vec![],
        }
    }
}

fn right_str(input: &str) -> Result<&str> {
    let (_, r) = input.split_once(": ").ok_anyhow()?;
    Ok(r)
}

fn parse_input(input: &str) -> Result<Computer> {
    let mut it = input.lines();

    // read registers
    let reg_a = right_str(it.next().ok_anyhow()?)?.parse()?;
    let reg_b = right_str(it.next().ok_anyhow()?)?.parse()?;
    let reg_c = right_str(it.next().ok_anyhow()?)?.parse()?;

    // skip blank line
    it.next();

    // read program
    let program_str = right_str(it.next().ok_anyhow()?)?;
    let mut program = vec![];
    for code_str in program_str.split(",") {
        let code = code_str.parse()?;
        program.push(code);
    }

    Ok(Computer {
        reg_a,
        reg_b,
        reg_c,
        program,
        ip: 0,
        output: vec![],
    })
}

impl Computer {
    /// Combo operands 0 through 3 represent literal values 0 through 3.
    /// Combo operand 4 represents the value of register A.
    /// Combo operand 5 represents the value of register B.
    /// Combo operand 6 represents the value of register C.
    /// Combo operand 7 is reserved and will not appear in valid programs.
    fn combo_operand(&self, operand: i64) -> Option<i64> {
        match operand {
            0..=3 => Some(operand),
            4 => Some(self.reg_a),
            5 => Some(self.reg_b),
            6 => Some(self.reg_c),
            _ => None,
        }
    }

    /// The adv instruction (opcode 0) performs division. The numerator is the value in the A register.
    /// The denominator is found by raising 2 to the power of the instruction's combo operand.
    /// (So, an operand of 2 would divide A by 4 (2^2); an operand of 5 would divide A by 2^B.)
    /// The result of the division operation is truncated to an integer and then written to the A register.
    fn adv(&mut self, operand: i64) {
        let operand = self.combo_operand(operand).unwrap();
        self.reg_a = self.reg_a >> operand;
        self.ip += 2;
    }

    /// The bdv instruction (opcode 6) works exactly like the adv instruction except that the
    /// result is stored in the B register. (The numerator is still read from the A register.)
    fn bdv(&mut self, operand: i64) {
        let operand = self.combo_operand(operand).unwrap();
        self.reg_b = self.reg_a >> operand;
        self.ip += 2;
    }

    /// The cdv instruction (opcode 7) works exactly like the adv instruction except that the
    /// result is stored in the C register. (The numerator is still read from the A register.)
    fn cdv(&mut self, operand: i64) {
        let operand = self.combo_operand(operand).unwrap();
        self.reg_c = self.reg_a >> operand;
        self.ip += 2;
    }

    /// The bxl instruction (opcode 1) calculates the bitwise XOR of register B
    /// and the instruction's literal operand, then stores the result in register B.
    fn bxl(&mut self, operand: i64) {
        self.reg_b ^= operand;
        self.ip += 2;
    }

    /// The bst instruction (opcode 2) calculates the value of its combo operand modulo 8
    /// (thereby keeping only its lowest 3 bits), then writes that value to the B register.
    fn bst(&mut self, operand: i64) {
        let x = self.combo_operand(operand).unwrap();
        self.reg_b = x & 0x7;
        self.ip += 2;
    }

    /// The jnz instruction (opcode 3) does nothing if the A register is 0.
    /// However, if the A register is not zero, it jumps by setting the
    /// instruction pointer to the value of its literal operand;
    /// if this instruction jumps, the instruction pointer is not
    /// increased by 2 after this instruction.
    fn jnz(&mut self, operand: i64) {
        if self.reg_a == 0 {
            self.ip += 2;
        } else {
            self.ip = operand as usize;
        }
    }

    /// The bxc instruction (opcode 4) calculates the bitwise XOR of register B and register C,
    /// then stores the result in register B. (For legacy reasons, this instruction
    /// reads an operand but ignores it.)
    fn bxc(&mut self, _operand: i64) {
        self.reg_b = self.reg_b ^ self.reg_c;
        self.ip += 2;
    }

    /// The out instruction (opcode 5) calculates the value of its combo operand modulo 8,
    /// then outputs that value. (If a program outputs multiple values, they are separated by commas.)
    fn out(&mut self, operand: i64) {
        // TODO: check for any negative numbers
        let x = self.combo_operand(operand).unwrap() & 0x7;
        self.output.push(x as u8);
        if self.output.len() > 100 {
            panic!("output too long");
        }

        self.ip += 2;
    }

    fn step(&mut self) {
        let inst = self.program[self.ip] as i64;
        let operand = self.program[self.ip + 1] as i64;
        match inst {
            0 => self.adv(operand),
            1 => self.bxl(operand),
            2 => self.bst(operand),
            3 => self.jnz(operand),
            4 => self.bxc(operand),
            5 => self.out(operand),
            6 => self.bdv(operand),
            7 => self.cdv(operand),
            _ => panic!("unexpected instruction {inst}"),
        }
    }

    fn run_program(&mut self) {
        while self.ip < self.program.len() {
            self.step();
        }
    }

    fn format_output(&self) -> String {
        self.output.iter().join(",")
    }
}

fn part1(mut computer: Computer) -> Result<String> {
    computer.run_program();
    Ok(computer.format_output())
}

fn part2(computer: &Computer) -> Result<i64> {
    let mut candidate = computer.clone();

    for init_a in 0.. {
        if init_a % 100_000_000 == 0 {
            println!("{init_a}...");
        }

        candidate.ip = 0;
        candidate.output.clear();
        candidate.reg_a = init_a;
        candidate.reg_b = computer.reg_b;
        candidate.reg_c = computer.reg_c;

        while candidate.ip < computer.program.len() {
            candidate.step();

            // break if output exceeds length of program
            if candidate.output.len() > computer.program.len() {
                break;
            }

            // break if last output differs from program
            if candidate.output.len() > 0 {
                let ix = candidate.output.len() - 1;
                if candidate.output[ix] != computer.program[ix] {
                    break;
                }
            }
        }

        // check end state
        if candidate.output == computer.program {
            return Ok(init_a);
        }
    }

    bail!("No solution found")
}

fn part_2_hardcoded(computer: Computer) -> Result<i64> {
    fn single_loop(a: i64) -> (i64, i64) {
        let b = a & 0x7;
        let b = b ^ 2;
        let c = a >> b;
        let b = b ^ c;
        let b = b ^ 3;
        let res = b & 0x7;
        let a = a >> 3;
        (res, a)
    }

    fn find_a_for(start:i64, target_out: i64, target_a: i64) -> i64 {
        (start..)
            .find(|i| {
                if i % 1_000_000_000 == 0 {
                    print!(".");
                }
                let (res, a) = single_loop(*i);
                res == target_out && a == target_a
            })
            .unwrap()
    }

    //let mut init_a = 0;
    println!("{:?}", computer.program);
    // verify first run
    let mut a = computer.reg_a;
    while a > 0 {
        let (res, a2) = single_loop(a);
        println!("{a} -> {res}");
        a = a2;
    }

    println!("-- search --");

    let mut target_a = 0;
    for n in computer.program.iter().rev().copied() {
        let start = 0 << 3;
        let target_out = n as i64;
        let a = find_a_for(start, target_out, target_a);
        target_a = a;
        println!("{a} -> {target_out}");
        //init_a = (init_a << 3) + a;

    }
    let test_computer = Computer {
        reg_a: target_a,
        reg_b: 0, 
        reg_c: 0,
        .. computer.clone()
    };
    println!("{:?}", part1(test_computer)?);

    Ok(target_a)
}

fn main() -> anyhow::Result<()> {
    let problem = parse_input(INPUT)?;
    println!("{problem:?}");

    let t1 = Instant::now();
    let res_part1 = part1(problem.clone())?;
    println!("Part 1 result is {res_part1} (took {:?})", t1.elapsed());

    let t2 = Instant::now();
    let res_part2_hardcoded = part_2_hardcoded(problem.clone())?;
    println!(
        "Part 2 result is {res_part2_hardcoded} (took {:?})",
        t2.elapsed()
    );
    let res_part2 = part2(&problem)?;
    println!("Part 2 result is {res_part2} (took {:?})", t2.elapsed());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
        Register A: 729
        Register B: 0
        Register C: 0

        Program: 0,1,5,4,3,0
    "};

    const EXAMPLE_P2: &str = indoc! {"
        Register A: 2024
        Register B: 0
        Register C: 0

        Program: 0,3,5,4,3,0
    "};

    // If register C contains 9, the program 2,6 would set register B to 1.
    #[test]
    fn case1() {
        let mut computer = Computer::new(0, 0, 9, vec![2, 6]);
        computer.run_program();
        assert_eq!(computer.reg_b, 1);
    }

    // If register A contains 10, the program 5,0,5,1,5,4 would output 0,1,2.
    #[test]
    fn case2() {
        let mut computer = Computer::new(10, 0, 0, vec![5, 0, 5, 1, 5, 4]);
        computer.run_program();
        assert_eq!(computer.format_output(), "0,1,2");
    }

    // If register A contains 2024, the program 0,1,5,4,3,0 would
    // output 4,2,5,6,7,7,7,7,3,1,0 and leave 0 in register A.
    #[test]
    fn case3() {
        let mut computer = Computer::new(2024, 0, 0, vec![0, 1, 5, 4, 3, 0]);
        computer.run_program();
        assert_eq!(computer.format_output(), "4,2,5,6,7,7,7,7,3,1,0");
        assert_eq!(computer.reg_a, 0);
    }

    // If register B contains 29, the program 1,7 would set register B to 26.
    #[test]
    fn case4() {
        let mut computer = Computer::new(0, 29, 0, vec![1, 7]);
        computer.run_program();
        assert_eq!(computer.reg_b, 26);
    }

    // If register B contains 2024 and register C contains 43690, the program 4,0 would set register B to 44354.
    #[test]
    fn case5() {
        let mut computer = Computer::new(0, 2024, 43690, vec![4, 0]);
        computer.run_program();
        assert_eq!(computer.reg_b, 44354);
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
        let output = part1(problem)?;
        assert_eq!(output, "4,6,3,5,6,3,5,2,1,0");
        Ok(())
    }

    #[test]
    fn part2_correct() -> Result<()> {
        let problem = parse_input(EXAMPLE_P2)?;
        let count = part2(&problem)?;
        assert_eq!(count, 117440);
        Ok(())
    }
}
