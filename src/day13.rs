use common::aoc::{load_input_bytes, print_result, print_time, run_many, run_once};

const C_COMMA: u8 = ',' as u8;
const C_X: u8 = 'x' as u8;
const C_ZERO: u8 = '0' as u8;
const C_NEWLINE: u8 = '\n' as u8;
const X: i64 = 0;

fn main() {
    let (input, dur_load) = run_once(|| load_input_bytes("day13"));

    print_time("Load", dur_load);

    let ((current, shuttles), dur_parse) = run_many(1000, || parse_input(&input));
    let (res_part1, dur_part1) = run_many(10000, || part1(current, &shuttles));
    let (res_part2, dur_part2) = run_many(1, || part2(&shuttles));

    print_result("P1", res_part1);
    print_result("P2", res_part2);

    print_time("Parse", dur_parse);
    print_time("P1", dur_part1);
    print_time("P2", dur_part2);
    print_time("Total", dur_parse + dur_part1 + dur_part2);
}

fn part1(current: i64, shuttles: &[Shuttle]) -> i64 {
    let mut winner = 0;
    let mut winner_min = i64::max_value();

    for Shuttle(id, _) in shuttles.iter() {
        if *id == X {
            continue;
        }

        let minutes = id - (current % id);
        if minutes < winner_min {
            winner = *id;
            winner_min = minutes;
        }
    }

    winner * winner_min
}

fn part2(shuttles: &[Shuttle]) -> i64 {
    let prod = shuttles
        .iter()
        .map(|Shuttle(id, _)| *id as i64)
        .product::<i64>();
    let mut sum = 0;

    for Shuttle(id, minutes) in shuttles.iter() {
        let remaining_minutes = *id - (minutes % *id);
        let p = prod / *id;

        sum += remaining_minutes * mod_inv(p, *id).unwrap() * p
    }

    sum % prod
}

fn parse_input(input: &[u8]) -> (i64, Vec<Shuttle>) {
    let mut minute = 0;
    let mut shuttles = Vec::with_capacity(32);

    let mut current = 0;
    let mut pos = 0;
    for c in input.iter() {
        if *c == C_COMMA || *c == C_NEWLINE {
            if current > 0 {
                if pos == 0 {
                    minute = current;
                } else {
                    shuttles.push(Shuttle(current, pos - 1));
                }

                current = 0;
            }

            pos += 1;
        } else if *c == C_X {
            // Do nothing
        } else {
            current = (current * 10) + (*c - C_ZERO) as i64;
        }
    }

    (minute, shuttles)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &[Shuttle] = &[
        Shuttle(7, 0),
        Shuttle(13, 1),
        Shuttle(59, 4),
        Shuttle(31, 6),
        Shuttle(19, 7),
    ];
    const EXAMPLE_2: &[Shuttle] = &[Shuttle(17, 0), Shuttle(13, 2), Shuttle(19, 3)];
    const EXAMPLE_3: &[Shuttle] = &[
        Shuttle(67, 0),
        Shuttle(7, 1),
        Shuttle(59, 2),
        Shuttle(61, 3),
    ];
    const EXAMPLE_4: &[Shuttle] = &[
        Shuttle(67, 0),
        Shuttle(7, 2),
        Shuttle(59, 3),
        Shuttle(61, 4),
    ];
    const EXAMPLE_5: &[Shuttle] = &[
        Shuttle(67, 0),
        Shuttle(7, 1),
        Shuttle(59, 3),
        Shuttle(61, 4),
    ];
    const EXAMPLE_6: &[Shuttle] = &[
        Shuttle(1789, 0),
        Shuttle(37, 1),
        Shuttle(47, 2),
        Shuttle(1889, 3),
    ];

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE_1), 1068781);
        assert_eq!(part2(EXAMPLE_2), 3417);
        assert_eq!(part2(EXAMPLE_3), 754018);
        assert_eq!(part2(EXAMPLE_4), 779210);
        assert_eq!(part2(EXAMPLE_5), 1261476);
        assert_eq!(part2(EXAMPLE_6), 1202161486);
    }
}

#[derive(Debug)]
struct Shuttle(i64, i64);

// Source: https://rosettacode.org/wiki/Chinese_remainder_theorem#Go

fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

fn mod_inv(x: i64, n: i64) -> Option<i64> {
    let (g, x, _) = egcd(x, n);
    if g == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}
