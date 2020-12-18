use common::aoc::{load_input_bytes, print_result, print_time, run_many, run_once};

const C_ZERO: u8 = '0' as u8;
const C_COMMA: u8 = ',' as u8;
const C_NEWLINE: u8 = '\n' as u8;

fn main() {
    let (input, dur_load) = run_once(|| load_input_bytes("day15"));

    print_time("Load", dur_load);

    let (starting_numbers, dur_parse) = run_many(1000, || parse_input(&input));
    let (res_part1, dur_part1) = run_many(1000, || part1(&starting_numbers));
    let (res_part2, dur_part2) = run_many(20, || part2(&starting_numbers));

    print_result("P1", res_part1);
    print_result("P2", res_part2);

    print_time("Parse", dur_parse);
    print_time("P1", dur_part1);
    print_time("P2", dur_part2);
    print_time("Total", dur_parse + dur_part1 + dur_part2);
}

fn part1(starting_numbers: &[usize]) -> usize {
    let mut cache = [0usize; 2020];

    for (i, n) in starting_numbers.iter().enumerate() {
        cache[*n] = i + 1;
    }

    let mut current = *starting_numbers.last().unwrap();

    for turn in starting_numbers.len() + 1..=2020 {
        let last_turn = turn - 1;
        let last = current;

        let cached = cache[last];
        if cached != 0 {
            current = last_turn - cached;
        } else {
            current = 0;
        }

        cache[last] = last_turn;
    }

    current
}

fn part2(starting_numbers: &[usize]) -> usize {
    let mut cache = vec![0usize; 30000000];

    for (i, n) in starting_numbers.iter().enumerate() {
        cache[*n] = i + 1;
    }

    let mut current = *starting_numbers.last().unwrap();

    for turn in starting_numbers.len() + 1..=30000000 {
        let last_turn = turn - 1;
        let last = current;

        let cached = unsafe { *cache.get_unchecked(last) };
        if cached != 0 {
            current = last_turn - cached;
        } else {
            current = 0;
        }

        unsafe { *cache.get_unchecked_mut(last) = last_turn }
    }

    current
}

fn parse_input(input: &[u8]) -> Vec<usize> {
    let mut results = Vec::with_capacity(8);
    let mut current = 0;
    for c in input.iter() {
        if *c == C_COMMA || *c == C_NEWLINE {
            results.push(current);
            current = 0;
        } else {
            current = (current * 10) + (*c - C_ZERO) as usize;
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &[usize] = &[0, 3, 6];
    const EXAMPLE_2: &[usize] = &[1, 3, 2];
    const EXAMPLE_3: &[usize] = &[2, 1, 3];
    const EXAMPLE_4: &[usize] = &[1, 2, 3];
    const EXAMPLE_5: &[usize] = &[2, 3, 1];
    const EXAMPLE_6: &[usize] = &[3, 2, 1];
    const EXAMPLE_7: &[usize] = &[3, 1, 2];

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE_1), 436);
        assert_eq!(part1(EXAMPLE_2), 1);
        assert_eq!(part1(EXAMPLE_3), 10);
        assert_eq!(part1(EXAMPLE_4), 27);
        assert_eq!(part1(EXAMPLE_5), 78);
        assert_eq!(part1(EXAMPLE_6), 438);
        assert_eq!(part1(EXAMPLE_7), 1836);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE_1), 175594);
        assert_eq!(part2(EXAMPLE_2), 2578);
        assert_eq!(part2(EXAMPLE_3), 3544142);
        assert_eq!(part2(EXAMPLE_4), 261214);
        assert_eq!(part2(EXAMPLE_5), 6895259);
        assert_eq!(part2(EXAMPLE_6), 18);
        assert_eq!(part2(EXAMPLE_7), 362);
    }
}
