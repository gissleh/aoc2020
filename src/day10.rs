use common::aoc::{print_result, print_time, run_many, run_once, load_input_bytes};

fn main() {
    let (input, dur_load) = run_once(|| load_input_bytes("day10"));

    print_time("Load", dur_load);

    let (list, dur_parse) = run_many(1000, || parse_input(&input));
    let (res_part1, dur_part1) = run_many(1000, || part1(&list));
    let (res_part2, dur_part2) = run_many(100000, || part2(&list));

    print_result("P1", res_part1);
    print_result("P2", res_part2);

    print_time("Parse", dur_parse);
    print_time("P1", dur_part1);
    print_time("P2", dur_part2);
    print_time("Total", dur_parse + dur_part1 + dur_part2);
}

fn part1(list: &[u32]) -> u32 {
    let mut threes = 0;
    let mut ones = 0;

    let mut p = 0;
    for n in list.iter() {
        let diff = n - p;
        if diff == 3 {
            threes += 1;
        } else if diff == 1 { // The comparison here could be omitted to halve the time, but that felt like cheating.
            ones += 1;
        }

        p = *n;
    }

    (threes + 1) * ones
}

fn part2(list: &[u32]) -> u64 {
    let mut sums = vec![1u64; list.len()];
    let mut sum = 0;

    let len = list.len();
    for i in 1..=len {
        let i = len - i;

        let item_i = list[i];
        let mut count = if i == len - 1 { 1 } else { 0 };
        for j in i+1..len {
            let item_j = list[j];
            if item_j - item_i > 3 {
                break;
            }

            count += sums[j];
        }

        sums[i] = count;

        if item_i <= 3 {
            sum += sums[i]
        }
    }

    sum
}

const NEWLINE: u8 = '\n' as u8;
const ZERO: u8 = '0' as u8;

fn parse_input(s: &[u8]) -> Vec<u32> {
    let mut res = Vec::with_capacity(1024);
    let mut current = 0;

    for b in s.iter() {
        if *b == NEWLINE {
            match res.binary_search(&current) {
                Ok(idx) => {
                    // Unreachable since the input is all unique numbers.
                    res.insert(idx, current);
                },
                Err(idx) => {
                    res.insert(idx, current);
                },
            }
            current = 0;
        } else {
            current *= 10;
            current += (b - ZERO) as u32;
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &[u32] = &[1, 4, 5, 6, 7, 10, 11, 12, 15, 16, 19];
    const EXAMPLE2: &[u32] = &[1, 2, 3, 4, 7, 8, 9, 10, 11,14, 17, 18, 19, 20, 23, 24, 25, 28, 31, 32, 33, 34, 35, 38, 39, 42, 45, 46, 47, 48, 49];

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE1), 7*5);
        assert_eq!(part1(EXAMPLE2), 22*10);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE1), 8);
        assert_eq!(part2(EXAMPLE2), 19208);
    }
}
