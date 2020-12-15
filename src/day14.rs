use common::aoc::{load_input, print_result, print_time, run_many, run_once};
use std::collections::BTreeMap;
use rustc_hash::FxHashMap;

const ZERO_U64: u64 = '0' as u64;

fn main() {
    let (input, dur_load) = run_once(|| load_input("day14"));

    print_time("Load", dur_load);

    let (instructions, dur_parse) = run_many(1000, || parse_input(&input));
    let (res_part1, dur_part1) = run_many(1000, || part1(&instructions));
    let (res_part2, dur_part2) = run_many(1000, || part2(&instructions));

    print_result("P1", res_part1);
    print_result("P2", res_part2);

    print_time("Parse", dur_parse);
    print_time("P1", dur_part1);
    print_time("P2", dur_part2);
    print_time("Total", dur_parse + dur_part1 + dur_part2);
}

fn part1(instructions: &[Instruction]) -> u64 {
    let mut memory = [0u64; 65536];
    let mut or_mask = 0u64;
    let mut and_mask = 0u64;

    for instruction in instructions.iter() {
        match instruction {
            Instruction::Mask{new_or_mask, new_and_mask, new_flippers: _} => {
                or_mask = *new_or_mask;
                and_mask = *new_and_mask;
            },
            Instruction::Memory(idx, value) => {
                memory[*idx as usize] = (*value | or_mask) & and_mask;
            }
        }
    }

    memory.iter().cloned().sum()
}

fn part2(instructions: &[Instruction]) -> u64 {
    let mut memory = FxHashMap::default();
    let mut or_mask = 0u64;
    let mut flippers = Vec::with_capacity(36);
    let mut flipper_combos = 0u64;

    for instruction in instructions.iter() {
        match instruction {
            Instruction::Mask{new_or_mask, new_and_mask: _, new_flippers} => {
                or_mask = *new_or_mask;
                flippers.clear();
                flipper_combos = 1;
                let mut current_bit = 34359738368u64; // 2^35
                while current_bit > 0 {
                    if new_flippers & current_bit == current_bit {
                        flippers.push(current_bit);
                        flipper_combos *= 2;
                    }

                    current_bit >>= 1;
                }
            },
            Instruction::Memory(idx, value) => {
                let base_idx = idx | or_mask;

                memory.insert(base_idx, *value);

                for current_flip in 1..=flipper_combos {
                    let mut idx = base_idx;
                    for flip_pos in 0..(flippers.len() as u64) {
                        let flip_pos_mask = 1 << flip_pos;
                        if flip_pos_mask & current_flip == flip_pos_mask {
                            idx |= flippers[flip_pos as usize];
                        } else {
                            idx &= 68719476735u64 ^ flippers[flip_pos as usize];
                        }
                    }

                    memory.insert(idx, *value);
                }
            }
        }
    }

    memory.values().cloned().sum()
}

#[derive(Debug)]
enum Instruction {
    Mask{new_or_mask: u64, new_and_mask: u64, new_flippers: u64},
    Memory(u64, u64),
}

fn parse_input(input: &str) -> Vec<Instruction> {
    let mut instructions = Vec::with_capacity(128);

    for line in input.lines() {
        if line.len() < 2 {
            break;
        }

        let split_idx = line.find(" = ").unwrap();
        let left = &line[..split_idx];
        let right = &line[split_idx + 3..];

        if left == "mask" {
            let mut current_bit = 34359738368u64; // 2^35
            let mut new_or_mask = 0u64;
            let mut new_and_mask = 68719476735u64; // 2^36 - 1
            let mut new_flippers = 0u64;
            for c in right.chars() {
                match c {
                    '1' => { new_or_mask |= current_bit; }
                    '0' => { new_and_mask ^= current_bit; }
                    'X' => { new_flippers |= current_bit; }
                    _ => panic!("bad bit")
                }

                current_bit >>= 1;
            }

            instructions.push(Instruction::Mask{ new_or_mask, new_and_mask, new_flippers });
        } else {
            instructions.push(Instruction::Memory(
                parse_u64(&left[4..left.len() - 1]),
                parse_u64(right),
            ));
        }
    }

    instructions
}

fn parse_u64(s: &str) -> u64 {
    let mut res = 0;

    for c in s.chars() {
        res *= 10;
        res += (c as u64) - ZERO_U64;
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_2: &str = "mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1";

    #[test]
    fn test_part2() {
        let instructions = parse_input(EXAMPLE_2);

        assert_eq!(part2(&instructions), 208);
    }
}