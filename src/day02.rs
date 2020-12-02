use common::aoc::{load_input, run_many, print_time, print_result};

fn main() {
    let input = load_input("day02");

    let (list, dur_parse) = run_many(1000, || parse_input(&input));
    let (res_part1, dur_part1) = run_many(100000, || part1(&list));
    let (res_part2, dur_part2) = run_many(100000, || part2(&list));

    print_result("P1", res_part1);
    print_result("P2", res_part2);

    print_time("Parse", dur_parse);
    print_time("P1", dur_part1);
    print_time("P2", dur_part2);
    print_time("Total", dur_parse + dur_part1 + dur_part2);
}

fn part1(lines: &[(u32, u32, u8, Vec<u8>)]) -> u32 {
    let mut count = 0;

    for (min, max, pwd_char, password) in lines.iter() {
        let mut cc: u32 = 0;
        for c in password.iter() {
            if *c == *pwd_char {
                cc += 1;
            }
        }

        if cc >= *min && cc <= *max {
            count += 1;
        }
    }

    count
}

fn part2(lines: &[(u32, u32, u8, Vec<u8>)]) -> u32 {
    let mut count = 0;

    for (first, second, pwd_char, password) in lines.iter() {
        let f = password[(*first - 1) as usize] == *pwd_char;
        let s = password[(*second - 1) as usize] == *pwd_char;

        if f != s {
            count += 1;
        }
    }

    count
}

fn parse_input(input: &str) -> Vec<(u32, u32, u8, Vec<u8>)> {
    let mut list= Vec::with_capacity(128);

    for line in input.lines() {
        let mut min = 0u32;
        let mut max = 0u32;
        let mut pwd_char = 0u8;
        let mut password: Vec<u8> = Vec::with_capacity(line.len());
        let mut state = 0;

        for char in line.chars() {
            if char == '-' || char == ':' {
                state += 1;
                continue;
            }
            if char == ' ' {
                continue;
            }

            if char >= '0' && char <= '9' {
                if state == 0 {
                    min = (min * 10) + (char as u8 - '0' as u8) as u32
                } else {
                    max = (max * 10) + (char as u8 - '0' as u8) as u32
                }
            } else {
                if state == 1 {
                    pwd_char = char as u8;
                } else {
                    password.push(char as u8);
                }
            }
        }

        list.push((min, max, pwd_char, password));
    }

    list
}

