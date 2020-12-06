use common::aoc::{load_input, print_result, print_time, run_many};

const A: usize = 'a' as usize;
const A_U32: u32 = 'a' as u32;

fn main() {
    let input = load_input("day06");

    let (gs, dur_parse) = run_many(1000, || GroupSet::parse(&input));

    let (res_part1, dur_part1) = run_many(100000, || part1(&gs));
    let (res_part2, dur_part2) = run_many(100000, || part2(&gs));

    print_result("P1", res_part1);
    print_result("P2", res_part2);

    print_time("Parse", dur_parse);
    print_time("P1", dur_part1);
    print_time("P2", dur_part2);
    print_time("Total", dur_parse + dur_part1 + dur_part2);

    let (list, dur_parse_alt) = run_many(1000, || parse_input_alt(&input));

    let (res_part1_alt, dur_part1_alt) = run_many(100000, || part1_alt(&list));
    let (res_part2_alt, dur_part2_alt) = run_many(100000, || part2_alt(&list));

    print_result("P1 ALT", res_part1_alt);
    print_result("P2 ALT", res_part2_alt);

    print_time("Parse ALT", dur_parse_alt);
    print_time("P1 ALT", dur_part1_alt);
    print_time("P2 ALT", dur_part2_alt);
    print_time("Total ALT", dur_parse_alt + dur_part1_alt + dur_part2_alt);
}

fn part1(gs: &GroupSet) -> u32 {
    let mut count = 0;
    let mut buf = [false; 26];

    for g in gs.groups.iter() {
        let start = g.pos;
        let end = start + g.len;

        for a in gs.answers[start..end].iter() {
            buf[*a] = true;
        }

        let mut group_count = 0;
        for elem in buf.iter_mut() {
            if *elem {
                group_count += 1;
            }

            *elem = false;
        }

        count += group_count;
    }

    count
}

fn part2(gs: &GroupSet) -> u32 {
    let mut count = 0;
    let mut buf = [0usize; 26];

    for g in gs.groups.iter() {
        let start = g.pos;
        let end = start + g.len;

        for a in gs.answers[start..end].iter() {
            buf[*a] += 1;
        }

        let mut group_count = 0;
        for elem in buf.iter_mut() {
            if *elem == g.size {
                group_count += 1;
            }

            *elem = 0;
        }

        count += group_count;
    }

    count
}

fn part1_alt(a: &[usize]) -> u32 {
    let mut count = 0;
    let mut buf = [false; 26];

    for n in a.iter() {
        match *n {
            28 => {
                for elem in buf.iter_mut() {
                    *elem = false;
                }
            }
            27 => {}
            _ => {
                if !buf[*n] {
                    count += 1;
                    buf[*n] = true;
                }
            }
        }
    }

    count
}

fn part2_alt(a: &[usize]) -> u32 {
    let mut count = 0;
    let mut buf = [0usize; 26];
    let mut group_size = 0usize;

    for n in a.iter() {
        match *n {
            28 => {
                for elem in buf.iter_mut() {
                    if *elem == group_size {
                        count += 1;
                    }

                    *elem = 0;
                }

                group_size = 0;
            }
            27 => {
                group_size += 1;
            }
            _ => {
                buf[*n] += 1;
            }
        }
    }

    count
}

fn parse_input_alt(s: &str) -> Vec<usize> {
    let mut res = Vec::with_capacity(s.len());

    let mut p = ' ';
    for c in s.chars() {
        if c == '\n' {
            if p == '\n' {
                res.push(28);
            } else {
                res.push(27);
            }
        } else {
            res.push((c as usize) - A)
        }

        p = c;
    }

    res.push(28);

    res
}

struct GroupSet {
    groups: Vec<Group>,
    answers: Vec<usize>,
}

#[derive(Debug)]
struct Group {
    pos: usize,
    len: usize,
    size: usize,
}

impl GroupSet {
    pub fn parse(s: &str) -> GroupSet {
        let mut groups = Vec::with_capacity(s.len() / 4);
        let mut group_idx = 0usize;
        let mut answers = Vec::with_capacity(s.len());

        groups.push(Group {
            len: 0,
            pos: 0,
            size: 0,
        });

        let mut p = ' ';
        for c in s.chars() {
            if c == '\n' {
                if p == '\n' {
                    groups[group_idx].len = answers.len() - groups[group_idx].pos;
                    groups.push(Group {
                        pos: answers.len(),
                        size: 0,
                        len: 0,
                    });
                    group_idx += 1;
                } else {
                    groups[group_idx].size += 1;
                }
            } else {
                answers.push((c as usize) - A);
            }

            p = c;
        }

        groups[group_idx].len = answers.len() - groups[group_idx].pos;

        GroupSet { groups, answers }
    }
}