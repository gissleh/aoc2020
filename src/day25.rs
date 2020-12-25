use common::aoc::{load_input, print_result, print_time, run_many, run_once};
use common::parsers::parse_u64;

const START_VALUE: u64 = 1;
const SUBJECT_NUMBER: u64 = 7;
const MODULO: u64 = 20201227;

fn main() {
    let (input, dur_load) = run_once(|| load_input("day25"));

    print_time("Load", dur_load);

    let ((card_public_key, door_public_key), dur_parse) = run_many(1000, || parse_input(&input));
    let (res_part1, dur_part1) = run_many(100, || part1(card_public_key, door_public_key));

    println!("Input (Card): {}", card_public_key);
    println!("Input (Door): {}", door_public_key);

    print_result("P1", res_part1);

    print_time("Parse", dur_parse);
    print_time("P1", dur_part1);
    print_time("Total", dur_parse + dur_part1);
}

fn part1(card_public_key: u64, door_public_key: u64) -> u64 {
    let card_loop_size = find_loop_size(card_public_key);
    let encryption_key = transform_key(door_public_key, card_loop_size);

    encryption_key
}

fn find_loop_size(public_key: u64) -> u64 {
    let mut loop_size = 0;
    let mut value = START_VALUE;

    while value != public_key {
        value = (value * SUBJECT_NUMBER) % MODULO;
        loop_size += 1;
    }

    loop_size
}

fn transform_key(public_key: u64, loop_size: u64) -> u64 {
    let mut value = 1;
    for _ in 0..loop_size {
        value = (value * public_key) % MODULO;
    }

    value
}

fn parse_input(input: &str) -> (u64, u64) {
    let newline_pos = input.find('\n').unwrap();
    let card_public_key = parse_u64(&input[..newline_pos]);
    let door_public_key = parse_u64(&input[newline_pos + 1..input.len() - 1]);

    (card_public_key, door_public_key)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_loop_size() {
        assert_eq!(find_loop_size(5764801), 8);
        assert_eq!(find_loop_size(17807724), 11);
    }

    #[test]
    fn test_transform() {
        assert_eq!(transform_key(17807724, 8), 14897079);
        assert_eq!(transform_key(5764801, 11), 14897079);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(5764801, 17807724), 14897079);
    }
}
