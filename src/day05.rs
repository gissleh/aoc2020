use common::aoc::{load_input, print_result, print_time, run_many};

const R: u8 = 'R' as u8;
const B: u8 = 'B' as u8;

fn main() {
    let input = load_input("day05");

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

fn part1(list: &[u8]) -> usize {
    let mut highest_id = 0;

    for i in 0..list.len() / 10 {
        let s = i * 10;
        let e = s + 10;
        let id = pass_id(&list[s..e]);
        if id > highest_id {
            highest_id = id;
        }
    }

    highest_id
}

fn part2(list: &[u8]) -> usize {
    let mut taken = vec![false; 128 * 8];

    for i in 0..list.len() / 10 {
        let s = i * 10;
        let e = s + 10;
        taken[pass_id(&list[s..e])] = true;
    }

    for i in 1..taken.len() - 1 {
        if taken[i - 1] && !taken[i] && taken[i + 1] {
            return i;
        }
    }

    panic!("position not found")
}

fn parse_input(input: &str) -> Vec<u8> {
    let mut vec = Vec::with_capacity(input.len());

    for ch in input.chars() {
        if ch != '\n' {
            vec.push(ch as u8);
        }
    }

    vec
}

fn pass_id(pass: &[u8]) -> usize {
    (pass[0] == B) as usize * 512
        + (pass[1] == B) as usize * 256
        + (pass[2] == B) as usize * 128
        + (pass[3] == B) as usize * 64
        + (pass[4] == B) as usize * 32
        + (pass[5] == B) as usize * 16
        + (pass[6] == B) as usize * 8
        + (pass[7] == R) as usize * 4
        + (pass[8] == R) as usize * 2
        + (pass[9] == R) as usize * 1
}

#[cfg(test)]
mod tests {
    use super::*;

    fn p(s: &str) -> Vec<u8> {
        s.chars().map(|c| c as u8).collect()
    }

    #[test]
    fn test_pass_id() {
        assert_eq!(pass_id(&p("BFFFBBFRRR")), 567);
        assert_eq!(pass_id(&p("FFFBBBFRRR")), 119);
        assert_eq!(pass_id(&p("BBFFBBFRLL")), 820);
    }
}
