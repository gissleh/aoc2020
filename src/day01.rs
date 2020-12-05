use common::aoc::{load_input, print_result, print_time, run_many};

fn main() {
    let input = load_input("day01");

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

fn part1(input: &[u32]) -> u32 {
    let mut set = [false; 2020];

    for n in input.iter().map(|n| *n as usize) {
        // If the set has the number that would fit with this one.
        if set[2020 - n] {
            return ((2020 - n) * n) as u32;
        }

        // Add it to set.
        set[n] = true;
    }

    0
}

fn part2(input: &[u32]) -> u32 {
    let mut min: u32 = 2020;

    for (i, n1) in input.iter().enumerate() {
        if *n1 < min {
            min = *n1
        }
        if *n1 + min >= 2020 {
            continue;
        }

        for (j, n2) in input[i..].iter().enumerate() {
            let n12 = *n1 + *n2;
            if n12 > 2020 - min {
                continue;
            }

            for n3 in input[i + j..].iter() {
                if n12 + *n3 == 2020 {
                    return *n1 * *n2 * *n3;
                }
            }
        }
    }

    0
}

fn parse_input(input: &str) -> Vec<u32> {
    let mut list: Vec<u32> = Vec::with_capacity(128);

    let mut sum: u32 = 0;
    for ch in input.chars() {
        if ch == '\n' || ch == '\r' {
            if sum > 0 {
                list.push(sum);
            }
            sum = 0;
            continue;
        }

        sum *= 10;
        sum += ch as u32 - '0' as u32;
    }

    list
}
