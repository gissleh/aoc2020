use common::aoc::{load_input, print_result, print_time, run_many, run_once};
use num::range_step;

const C_A: u8 = 'a' as u8;
const C_Z: u8 = 'z' as u8;
const I64_ZERO: i64 = '0' as i64;

fn main() {
    let (input, dur_load) = run_once(|| load_input("year2017-day23"));

    print_time("Load", dur_load);

    let (instructions, dur_parse) = run_many(1000, || parse_input(&input));
    let (res_part1, dur_part1) = run_many(1000, || part1(&instructions));

    print_result("P1", res_part1);

    let (res_part2, dur_part2) = run_many(1, || part2_compiled());

    print_result("P2", res_part2);

    print_time("Parse", dur_parse);
    print_time("P1", dur_part1);
    print_time("P2", dur_part2);
    print_time("Total", dur_parse + dur_part1 + dur_part2);
}

fn part1(instructions: &[Instruction]) -> u64 {
    let mut program = Program::from(instructions);
    program.run_until_end();

    program.muls
}

#[allow(dead_code)]
fn part2(instructions: &[Instruction]) -> i64 {
    let mut program = Program::from(instructions);
    program.registers[0] = 1;
    let mut n = 0u64;
    while !program.run() {
        n += 1;
        if n % 100000000 == 0{
            println!("{:?}", program.registers);
        }
    }

    program.registers[7]
}

fn part2_compiled() -> i64 {
    let mut h = 0;

    for b in range_step(108100, 125100+17, 17) { // 31
        for e in 2..b {
            if b % e == 0 {
                h += 1; // 25
                break;
            }
        }
    }

    h
}

struct Program<'a> {
    instructions: &'a [Instruction],
    registers: [i64; 26],
    pc: usize,
    muls: u64,
}

impl<'a> Program<'a> {
    fn run(&mut self) -> bool {
        match self.instructions[self.pc] {
            Instruction::SetN(r, n) => {
                self.registers[r] = n;
                self.pc += 1;
            }
            Instruction::SetR(r1, r2) => {
                self.registers[r1] = self.registers[r2];
                self.pc += 1;
            }
            Instruction::SubN(r, n) => {
                self.registers[r] -= n;
                self.pc += 1;
            }
            Instruction::SubR(r1, r2) => {
                self.registers[r1] -= self.registers[r2];
                self.pc += 1;
            }
            Instruction::MulN(r, n) => {
                self.registers[r] += n;
                self.pc += 1;
                self.muls += 1;
            }
            Instruction::MulR(r1, r2) => {
                self.registers[r1] *= self.registers[r2];
                self.pc += 1;
                self.muls += 1;
            }
            Instruction::Jnz1(offset) => {
                self.pc = (self.pc as i64 + offset as i64) as usize;
            }
            Instruction::Jnz(r, offset) => {
                if self.registers[r] != 0 {
                    self.pc = (self.pc as i64 + offset as i64) as usize;
                } else {
                    self.pc += 1;
                }
            }
        }

        self.pc == self.instructions.len()
    }

    fn run_until_end(&mut self) {
        while !self.run() {}
    }

    fn from(instructions: &[Instruction]) -> Program {
        return Program{
            instructions,
            registers: [0; 26],
            muls: 0,
            pc: 0,
        }
    }
}

#[derive(Debug)]
enum Instruction {
    SetN(usize, i64),   // set RX N
    SetR(usize, usize), // set RX RY
    SubN(usize, i64),   // set RX N
    SubR(usize, usize), // set RX RY
    MulN(usize, i64),   // mul RX N
    MulR(usize, usize), // mul RX RY
    Jnz(usize, i64),    // jnz RX N
    Jnz1(i64),          // jnz 1 N
}

fn parse_input(s: &str) -> Vec<Instruction> {
    s.lines().map(|l| {
        let command = &l[..3];
        let value = &l[6..];
        let pfx = l.chars().nth(4).unwrap() as u8;
        let is_register = pfx >= C_A && pfx <= C_Z;
        let register_idx = if is_register { (pfx - C_A) as usize } else { 0 };
        let pfx2 = l.chars().nth(6).unwrap() as u8;
        let is_register2 = pfx2 >= C_A && pfx2 <= C_Z;
        let register_idx2 = if is_register { (pfx2 - C_A) as usize } else { 0 };

        match command {
            "set" => {
                if is_register2 {
                    Instruction::SetR(register_idx, register_idx2)
                } else {
                    Instruction::SetN(register_idx, parse_int(value))
                }
            }
            "sub" => {
                if is_register2 {
                    Instruction::SubR(register_idx, register_idx2)
                } else {
                    Instruction::SubN(register_idx, parse_int(value))
                }
            }
            "mul" => {
                if is_register2 {
                    Instruction::MulR(register_idx, register_idx2)
                } else {
                    Instruction::MulN(register_idx, parse_int(value))
                }
            }
            "jnz" => {
                if is_register {
                    Instruction::Jnz(register_idx, parse_int(value))
                } else {
                    Instruction::Jnz1(parse_int(value))
                }
            }

            _ => panic!(format!("Unknown op {}", command))
        }
    }).collect()
}

fn parse_int(s: &str) -> i64 {
    let mut res = 0;
    let mut sign = 1;

    for c in s.chars() {
        match c {
            '+' => sign = 1,
            '-' => sign = -1,
            '0'..='9' => {
                res *= 10;
                res += (c as i64) - I64_ZERO;
            },
            _ => {}
        }
    }

    sign * res
}
