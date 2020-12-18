use common::aoc::{load_input, print_result, print_time, run_many, run_once};

fn main() {
    let (input, dur_load) = run_once(|| load_input("day08"));

    print_time("Load", dur_load);

    let (program, dur_parse) = run_many(1000, || Program::parse(&input));
    let (res_part1, dur_part1) = run_many(100000, || program.part1());
    let (res_part2, dur_part2) = run_many(10000, || program.part2());

    print_result("P1", res_part1);
    print_result("P2", res_part2);

    print_time("Parse", dur_parse);
    print_time("P1", dur_part1);
    print_time("P2", dur_part2);
    print_time("Total", dur_parse + dur_part1 + dur_part2);
}

const ZERO: i32 = '0' as i32;
const NOP: u32 = 0;
const ACC: u32 = 1;
const JMP: u32 = 2;

#[derive(Debug)]
struct Instruction(u32, i32);

struct Program {
    instructions: Vec<Instruction>,
}

impl Program {
    fn part1(&self) -> i32 {
        let mut acc = 0;
        let mut pc = 0;
        let mut has_seen = vec![false; self.instructions.len()];

        loop {
            if has_seen[pc] {
                return acc;
            }
            has_seen[pc] = true;

            let Instruction(op, n) = self.instructions[pc];

            match op {
                NOP => pc += 1,
                ACC => {
                    acc += n;
                    pc += 1
                }
                JMP => (pc = (pc as i32 + n) as usize),
                _ => {}
            }
        }
    }

    fn part2(&self) -> i32 {
        let target = self.instructions.len();
        let mut acc = 0;
        let mut pc = 0;
        let mut seen_log = Vec::with_capacity(512);
        let mut has_seen = vec![false; self.instructions.len()];

        let mut checkpoint = (acc, pc);
        let mut active = false;
        let mut failed = false;

        loop {
            let Instruction(mut op, n) = self.instructions[pc];

            if op != ACC && n != 0 && !active && !failed {
                checkpoint = (acc, pc);
                active = true;

                op = if op == JMP { NOP } else { JMP };
                seen_log.push(pc);
            } else if active {
                seen_log.push(pc);
            }

            failed = false;

            if has_seen[pc] {
                acc = checkpoint.0;
                pc = checkpoint.1;

                active = false;
                failed = true;

                for i in seen_log.iter() {
                    has_seen[*i] = false;
                }
                seen_log.clear();
                continue;
            }
            has_seen[pc] = true;

            match op {
                NOP => pc += 1,
                ACC => {
                    acc += n;
                    pc += 1
                }
                JMP => (pc = (pc as i32 + n) as usize),
                _ => {}
            }

            if pc == target {
                return acc;
            }
        }
    }

    pub fn parse(s: &str) -> Program {
        let mut instructions = Vec::with_capacity(1024);

        for line in s.lines() {
            if line.len() == 0 {
                break;
            }

            let n = parse_int(&line[4..]);

            match &line[0..3] {
                "nop" => instructions.push(Instruction(NOP, n)),
                "acc" => instructions.push(Instruction(ACC, n)),
                "jmp" => instructions.push(Instruction(JMP, n)),
                _ => panic!(format!("Unknown instruction: {}", line)),
            }
        }

        Program { instructions }
    }
}

fn parse_int(s: &str) -> i32 {
    let mut res = 0;
    let mut sign = 1;

    for c in s.chars() {
        match c {
            '+' => sign = 1,
            '-' => sign = -1,
            '0'..='9' => {
                res *= 10;
                res += (c as i32) - ZERO;
            }
            _ => {}
        }
    }

    sign * res
}
