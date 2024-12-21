use core::fmt;
use regex::Regex;

advent_of_code::solution!(17);

const ADV: usize = 0;
const BXL: usize = 1;
const BST: usize = 2;
const JNZ: usize = 3;
const BXC: usize = 4;
const OUT: usize = 5;
const BDV: usize = 6;
const CDV: usize = 7;

#[derive(Clone, Copy)]
struct Registers {
    a: usize,
    b: usize,
    c: usize,
}

impl fmt::Display for Registers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Register A: {}\nRegister B: {}\nRegister C: {}",
            self.a, self.b, self.c
        )
    }
}

fn combo(op: usize, reg: &Registers) -> usize {
    match op {
        0..=3 => op,
        4 => reg.a,
        5 => reg.b,
        6 => reg.c,
        _ => panic!("unsupported combo"),
    }
}

fn adv(lhs: usize, rhs: usize) -> usize {
    let base: usize = 2;
    lhs / base.pow(rhs as u32)
}

fn run_program(reg: &Registers, program: &[usize]) -> Vec<usize> {
    let mut output = Vec::new();
    let mut pc = 0;
    let mut reg = *reg;
    while pc < program.len() {
        let instruction = program[pc];
        pc += 1;
        let operand = program[pc];
        pc += 1;

        match instruction {
            ADV => {
                reg.a = adv(reg.a, combo(operand, &reg));
            }
            BXL => {
                reg.b ^= operand;
            }
            BST => {
                reg.b = combo(operand, &reg) % 8;
            }
            JNZ => {
                if reg.a != 0 {
                    pc = operand;
                }
            }
            BXC => {
                reg.b ^= reg.c;
            }
            OUT => {
                let out = combo(operand, &reg) % 8;
                output.push(out);
            }
            BDV => {
                reg.b = adv(reg.a, combo(operand, &reg));
            }
            CDV => {
                reg.c = adv(reg.a, combo(operand, &reg));
            }
            _ => panic!("Unsupportedinstruction"),
        }
    }
    output
}

pub fn part_one(input: &str) -> Option<String> {
    let re = Regex::new(
        r"Register A: (\d*)\s*Register B: (\d*)\s*Register C: (\d*)\s*Program:\s([0-9,]*)",
    )
    .unwrap();

    let mut reg = Registers { a: 0, b: 0, c: 0 };

    let mut program: Vec<usize> = vec![];

    if let Some(capture) = re.captures(input) {
        let (_, [a, b, c, program_string]) = capture.extract();
        reg.a = a.parse().expect("Unable to parse register A");
        reg.b = b.parse().expect("Unable to parse register B");
        reg.c = c.parse().expect("Unable to parse register C");

        program = program_string
            .split(",")
            .map(|x| x.parse().unwrap())
            .collect();
    }

    let output = run_program(&reg, &program);

    println!("{reg}\n\nPRG: {:?}", program);
    println!("{reg}\n\nOUT: {:?}", output);

    let out = output
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(",");

    Some(out)
}

pub fn part_two(input: &str) -> Option<u64> {
    let re = Regex::new(
        r"Register A: (\d*)\s*Register B: (\d*)\s*Register C: (\d*)\s*Program:\s([0-9,]*)",
    )
    .unwrap();

    let capture = re.captures(input).expect("failed to parse");

    let (_, [a, b, c, program_string]) = capture.extract();
    let a = a.parse().expect("Unable to parse register A");
    let b = b.parse().expect("Unable to parse register B");
    let c = c.parse().expect("Unable to parse register C");

    let init_reg = Registers { a, b, c };

    let program: Vec<usize> = program_string
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect();

    println!("{}\n\nProgram: {:?}", &init_reg, program);
    println!();
    println!();

    for instr in &program {
        print!("{:>3}", instr);
    }
    println!();
    for instr in &program {
        print!("{:03b}", instr);
    }
    println!();

    println!("Program length : {}", program.len());

    let program_bit_size = program.len() * 3;
    let base: usize = 2;
    let reg_a_max = base.pow(program_bit_size as u32) - 1;
    let reg_a_min = base.pow(program_bit_size as u32 - 2) - 1;
    println!(
        "MAX DEC: {:015}, OCT: {:o}, BIN: {:048b}",
        reg_a_max, reg_a_max, reg_a_max
    );
    println!(
        "MIN DEC: {:015}, OCT: {:o}, BIN: {:048b}",
        reg_a_min, reg_a_min, reg_a_min
    );

    let bit_window_search = 3;
    let start_bit = program_bit_size;
    let end_bit = program_bit_size - bit_window_search;
    let mut queue = Vec::new();
    queue.push((start_bit, end_bit, 0, 0));

    let mut results = Vec::new();

    while let Some((start_bit, end_bit, index, reg_a)) = queue.pop() {
        let bits_to_shift = start_bit - end_bit;
        let shift_max = (1 << bits_to_shift) - 1;
        println!("a:{:048b} {start_bit}-{end_bit} [{index}] ", reg_a);
        for i in 0..=shift_max {
            let reg_a = (i << end_bit) | reg_a;
            let reg = Registers {
                a: reg_a,
                b: 0,
                c: 0,
            };

            let output = run_program(&reg, &program);
            print!(
                "i:{:03} -> reg_a: {:048b} out: {}",
                i,
                reg_a,
                output
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join("")
            );

            if output.len() != program.len() {
                println!();
                continue;
            }

            // print!("a:{:048b} ", reg.a);
            // print!("o:");
            // for o in &output {
            //     print!("{}", o);
            // }
            // print!("\r");

            let mut output_match = true;
            if output == program {
                results.push(reg_a);
                println!();
                println!(
                    "{reg_a} -> {}",
                    output
                        .iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<_>>()
                        .join("")
                );
                continue;
            }
            for j in 0..=index {
                let index = output.len() - j - 1;
                if output[index] != program[index] {
                    output_match = false;
                }
            }
            if output_match {
                print!("***");
                let endbit = end_bit.saturating_sub(3);
                queue.push((start_bit - 3, endbit, index + 1, reg_a));
            }

            println!();
        }
    }
    results.sort();

    Some(results[0] as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("4,6,3,5,6,3,5,2,1,0".to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(117440));
    }
}
