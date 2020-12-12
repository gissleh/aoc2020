use common::aoc::{print_result, print_time, run_many, run_once, load_input_bytes};

const DIRECTIONS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
const C_FORWARD: u8 = 'F' as u8;
const C_LEFT: u8 = 'L' as u8;
const C_RIGHT: u8 = 'R' as u8;
const C_EAST: u8 = 'E' as u8;
const C_SOUTH: u8 = 'S' as u8;
const C_WEST: u8 = 'W' as u8;
const C_NORTH: u8 = 'N' as u8;
const C_ZERO: u8 = '0' as u8;
const C_NEWLINE: u8 = '\n' as u8;

fn main() {
    let (input, dur_load) = run_once(|| load_input_bytes("day12"));

    print_time("Load", dur_load);

    let (instructions, dur_parse) = run_many(1000, || parse_input(&input));
    let (res_part1, dur_part1) = run_many(100000, || part1(&instructions));
    //let (res_part2, dur_part2) = run_many(100000, || part2(&instructions));

    print_result("P1", res_part1);
    //print_result("P2", res_part2);

    print_time("Parse", dur_parse);
    print_time("P1", dur_part1);
    //print_time("P2", dur_part2);
    //print_time("Total", dur_parse + dur_part1 + dur_part2);
}

fn part1(instructions: &[Instruction]) -> i32 {
    let mut x = 0i32;
    let mut y = 0i32;
    let mut dir = 0usize;

    for instruction in instructions.iter() {
        match instruction {
            Instruction::Forward(l) => {
                x += DIRECTIONS[dir].0 * l;
                y += DIRECTIONS[dir].1 * l;
            }
            Instruction::Turn(off) => {
                dir = (dir + off) % 4;
            }
            Instruction::Move(mx, my) => {
                x += mx;
                y += my;
            }
        }
    }

    x.abs() + y.abs()
}

fn parse_input(input: &[u8]) -> Vec<Instruction> {
    let mut res = Vec::with_capacity(512);
    let mut current_length = 0;
    let mut current_inst = 0u8;
    let mut parsing_number = false;
    let minus_one = usize::max_value();

    for c in input.iter() {
        if parsing_number {
            if *c == C_NEWLINE {
                res.push(match current_inst {
                    C_FORWARD => Instruction::Forward(current_length),
                    C_LEFT => Instruction::Turn(minus_one * (current_length as usize / 90)),
                    C_RIGHT => Instruction::Turn(current_length as usize / 90),
                    C_EAST => Instruction::Move(current_length, 0),
                    C_SOUTH => Instruction::Move(0, current_length),
                    C_WEST => Instruction::Move(-current_length, 0),
                    C_NORTH => Instruction::Move(0, -current_length),
                    _ => panic!(format!("Unknown instruction: {}", current_inst as char)),
                });

                current_length = 0;
                parsing_number = false;
            } else {
                current_length *= 10;
                current_length += (*c - C_ZERO) as i32;
            }
        } else {
            current_inst = *c;
            parsing_number = true;
        }
    }

    res
}

#[derive(Debug)]
enum Instruction {
    Move(i32, i32),
    Turn(usize),
    Forward(i32)
}