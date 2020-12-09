use common::aoc::{print_result, print_time, run_many, run_once, load_input_bytes};

fn main() {
    let (input, dur_load) = run_once(|| load_input_bytes("day09"));

    print_time("Load", dur_load);

    let (list, dur_parse) = run_many(1000, || parse_input(&input));
    let (res_part1, dur_part1) = run_many(1000, || part1(&list, 25));
    let (res_part2, dur_part2) = run_many(100000, || part2(&list, res_part1));

    print_result("P1", res_part1);
    print_result("P2", res_part2);

    print_time("Parse", dur_parse);
    print_time("P1", dur_part1);
    print_time("P2", dur_part2);
    print_time("Total", dur_parse + dur_part1 + dur_part2);
}

fn part1(data: &[u64], preamble_length: usize) -> u64 {
    let mut preamble_pos = 0;
    let mut preamble = &data[0..preamble_length];

    for n in data[preamble_length..].iter() {
        let mut is_sum = false;

        'outer: for i in 0..preamble_length {
            let pre_i = preamble[i];

            for j in i+1..preamble_length {
                if pre_i + preamble[j] == *n {
                    is_sum = true;
                    break 'outer;
                }
            }
        }

        if !is_sum {
            return *n;
        }

        preamble_pos += 1;
        preamble = &data[preamble_pos..preamble_pos+preamble_length];
    }

    return 0;
}

fn part2(data: &[u64], target: u64) -> u64 {
    let mut lower = 0;
    let mut upper = 1;
    let mut sum = data[lower] + data[upper];

    while sum != target {
        let n = data[upper + 1];
        if sum + n <= target {
            sum += n;
            upper += 1;
        } else {
            sum -= data[lower];
            lower += 1;
        }
    }

    let mut smallest = target;
    let mut largest = 0;
    for n in data[lower..=upper].iter().cloned() {
        if n < smallest {
            smallest = n;
        }
        if n > largest {
            largest = n;
        }
    }

    smallest + largest
}

const NEWLINE: u8 = '\n' as u8;
const ZERO: u8 = '0' as u8;

fn parse_input(s: &[u8]) -> Vec<u64> {
    let mut res = Vec::with_capacity(1024);
    let mut current = 0;

    for b in s.iter() {
        if *b == NEWLINE {
            res.push(current);
            current = 0;
        } else {
            current *= 10;
            current += (b - ZERO) as u64;
        }
    }

    res
}


#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &'static[u64] = &[35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309, 576];

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE, 5), 127);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE, 127), 62);
    }
}
