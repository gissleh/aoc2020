use common::aoc::{load_input, print_result, print_time, run_many, run_once};

const SAMPLE_COMPONENTS: &[(u32, u32); 8] = &[
    (0, 2),
    (2, 2),
    (2, 3),
    (3, 4),
    (3, 5),
    (0, 1),
    (10, 1),
    (9, 10),
];

fn main() {
    let (input, dur_load) = run_once(|| load_input("year2017-day24"));

    print_time("Load", dur_load);

    let (components, dur_parse) = run_many(1000, || parse_input(&input));
    let (res_part1_sample, dur_part1_sample) = run_many(100000, || part1(&SAMPLE_COMPONENTS[..]));

    assert_eq!(res_part1_sample, 31);

    let (res_part1, dur_part1) = run_many(5, || part1(&components));
    let (res_part2, dur_part2) = run_many(5, || part2(&components));

    print_result("P1 (Sample)", res_part1_sample);
    print_result("P1", res_part1);
    print_result("P2", res_part2);

    print_time("Parse", dur_parse);
    print_time("P1 (Sample)", dur_part1_sample);
    print_time("P1", dur_part1);
    print_time("P2", dur_part2);
    //print_time("Total", dur_parse + dur_part1 + dur_part2);
}

fn part1(components: &[(u32, u32)]) -> u32 {
    let mut stack = Vec::with_capacity(64);
    let mut used = vec![false; components.len()];
    let mut max_strength = 0;

    for i in 0..components.len() {
        let (l, r) = components[i];
        let left_magnetic = l == 0;
        let right_magnetic = r == 0;
        if !left_magnetic && !right_magnetic {
            continue;
        }

        stack.push((i, 0usize, left_magnetic));
        used[i] = true;

        while !stack.is_empty() {
            let (i, pos, is_right) = stack.last().cloned().unwrap();
            let (li, ri) = components[i];
            let mut dead_end = true;

            let connector = if is_right {ri} else {li};

            for j in pos..components.len() {
                if used[j] {
                    continue;
                }
                let (l, r) = components[j];

                let left_match = connector == l;
                let right_match = connector == r;

                if left_match || right_match {
                    *stack.last_mut().unwrap() = (i, j + 1, is_right);
                    stack.push((j, 0usize, left_match));
                    used[j] = true;
                    dead_end = false;
                    break;
                }
            }

            if dead_end {
                if pos == 0 {
                    let strength = stack.iter()
                        .map(|(i, _, _)| &components[*i])
                        .map(|(l, r)| *l + *r)
                        .sum::<u32>();

                    if strength > max_strength {
                        max_strength = strength;
                    }
                }

                used[i] = false;
                stack.pop();
            }
        }

        used[i] = false;
    }

    max_strength
}

fn part2(components: &[(u32, u32)]) -> u32 {
    let mut stack = Vec::with_capacity(64);
    let mut used = vec![false; components.len()];
    let mut max_strength = 0;
    let mut max_length = 0;

    for i in 0..components.len() {
        let (l, r) = components[i];
        let left_magnetic = l == 0;
        let right_magnetic = r == 0;
        if !left_magnetic && !right_magnetic {
            continue;
        }

        stack.push((i, 0usize, left_magnetic));
        used[i] = true;

        while !stack.is_empty() {
            let (i, pos, is_right) = stack.last().cloned().unwrap();
            let (li, ri) = components[i];
            let mut dead_end = true;

            let connector = if is_right {ri} else {li};

            for j in pos..components.len() {
                if used[j] {
                    continue;
                }
                let (l, r) = components[j];

                let left_match = connector == l;
                let right_match = connector == r;

                if left_match || right_match {
                    *stack.last_mut().unwrap() = (i, j + 1, is_right);
                    stack.push((j, 0usize, left_match));
                    used[j] = true;
                    dead_end = false;
                    break;
                }
            }

            if dead_end {
                if pos == 0 && stack.len() >= max_length {
                    let strength = stack.iter()
                        .map(|(i, _, _)| &components[*i])
                        .map(|(l, r)| *l + *r)
                        .sum::<u32>();

                    if strength > max_strength || stack.len() > max_length {
                        max_strength = strength;
                        max_length = stack.len();
                    }
                }

                used[i] = false;
                stack.pop();
            }
        }

        used[i] = false;
    }

    max_strength
}


fn parse_input(input: &str) -> Vec<(u32, u32)> {
    input.lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.split('/'))
        .map(|mut t| (t.next().unwrap().parse().unwrap(), t.next().unwrap().parse().unwrap()))
        .collect()
}