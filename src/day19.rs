#[macro_use]
extern crate smallvec;

use common::aoc::{load_input, print_result, print_time, run_many, run_once};
use smallvec::SmallVec;
use common::parsers::parse_usize;

fn main() {
    let (input, dur_load) = run_once(|| load_input("day19"));

    print_time("Load", dur_load);

    let (input, dur_parse) = run_many(1000, || Input::parse(&input));
    let (res_part1, dur_part1) = run_many(100, || part1(&input));
    let (res_part2, dur_part2) = run_many(100, || part2(&input));

    print_result("P1", res_part1);
    print_result("P2", res_part2);

    print_time("Parse", dur_parse);
    print_time("P1", dur_part1);
    print_time("P2", dur_part2);
    print_time("Total", dur_parse + dur_part1 + dur_part2);
}

fn part1(input: &Input) -> u32 {
    parts_common(input, false)
}

fn part2(input: &Input) -> u32 {
    parts_common(input, true)
}

fn parts_common(input: &Input, is_part2: bool) -> u32 {
    let mut count = 0;

    let rules = if is_part2 { &input.rules_p2 } else { &input.rules };

    for line in input.lines() {
        if check_line(line, rules) {
            count += 1;
        }
    }

    count
}

fn check_line(line: &[u8], rules: &[Rule]) -> bool {
    let mut stack: SmallVec<[(usize, usize, usize, bool); 64]> = smallvec![
        (0, 0, 0, false),
    ];
    let mut line_pos = 0;
    let mut pass = true;

    #[cfg(test)] {
        //println!("Line: {}", String::from_utf8(Vec::from(line)).unwrap());
    }

    while stack.len() > 0 {
        let (rule_index, pos, old_line_pos, right) = stack.pop().unwrap();
        let rule = &rules[rule_index];

        #[cfg(test)] {
            let d = b'E';
            if stack.len() < 8 {
                println!("{}{} :: {:?} {} {} {}({}) {}", "  ".repeat(stack.len()), rule_index, rule, pos, right, line_pos, *line.get(line_pos).unwrap_or(&d) as char, pass);
            } else if stack.len() == 8 {
                println!("                ...");
            }
        }

        match rule {
            Rule::Character(b) => {
                if line_pos == line.len() {
                    pass = false;
                } else {
                    if line[line_pos] != *b {
                        pass = false;
                    }

                    line_pos += 1;
                }
            }
            Rule::SubRules(subs) => {
                if pass {
                    if pos < subs.len() {
                        stack.push((rule_index, pos + 1, old_line_pos, right));
                        stack.push((subs[pos], 0, line_pos, false));
                    }
                }
            }
            Rule::Disjunction(subs_left, subs_right) => {
                if right {
                    if pass {
                        if pos < subs_right.len() {
                            stack.push((rule_index, pos + 1, old_line_pos, true));
                            stack.push((subs_right[pos], 0, line_pos, false));
                        }
                    }
                } else {
                    if pass {
                        if pos < subs_left.len() {
                            stack.push((rule_index, pos + 1, old_line_pos, false));
                            stack.push((subs_left[pos], 0, line_pos, false));
                        }
                    } else {
                        pass = true;
                        line_pos = old_line_pos;
                        stack.push((rule_index, 0, old_line_pos, true));
                    }
                }
            }
            Rule::Nop => {}
        }
    }

    if pass && line_pos == line.len() {
        #[cfg(test)] {
            println!("PASS Line: {}", String::from_utf8(Vec::from(line)).unwrap());
        };
        true
    } else {
        #[cfg(test)] {
            println!("FAIL Line: {}", String::from_utf8(Vec::from(line)).unwrap());
        };
        false
    }
}

#[derive(Debug)]
struct Input {
    rules: Vec<Rule>,
    rules_p2: Vec<Rule>,
    sub_indexes: Vec<usize>,
    data: Vec<u8>,
    slices: Vec<(usize, usize)>,
}

impl Input {
    fn lines(&self) -> impl Iterator<Item=&[u8]> {
        self.slices.iter().map(move |(s, e)| &self.data[*s..*e])
    }

    fn parse(input: &str) -> Input {
        let mut rules = vec![Rule::Nop; 64];
        let mut data = Vec::with_capacity(64);
        let mut slices = Vec::with_capacity(64);
        let mut sub_indexes = Vec::with_capacity(64);

        let mut parsed_rules = false;

        for line in input.lines() {
            if line.len() == 0 {
                parsed_rules = true;
                continue;
            }

            // The input is ascii, so using the same indexes is safe.
            let bytes = line.as_bytes();

            if parsed_rules {
                let start = data.len();
                data.extend_from_slice(bytes);
                slices.push((start, data.len()));
            } else {
                let colon_pos = line.find(":").unwrap();
                let or_pos = line.find('|');
                let quote_pos = line.find('"');

                let rule_index = parse_usize(&line[..colon_pos]);
                while rules.len() <= rule_index {
                    rules.push(Rule::Nop);
                }

                if let Some(quote_pos) = quote_pos {
                    rules[rule_index] = Rule::Character(bytes[quote_pos + 1]);
                } else if let Some(or_pos) = or_pos {
                    rules[rule_index] = Rule::Disjunction(
                        line[colon_pos + 2..or_pos - 1].split(' ').map(|t| parse_usize(t)).collect(),
                        line[or_pos + 2..].split(' ').map(|t| parse_usize(t)).collect(),
                    );
                } else {
                    rules[rule_index] = Rule::SubRules(
                        line[colon_pos + 2..].split(' ').map(|t| parse_usize(t)).collect()
                    );

                    sub_indexes.push(sub_indexes.len());
                }
            }
        }

        let mut rules_p2 = rules.clone();
        /* let mut r8_left = SmallVec::new();
        let mut r8_right = SmallVec::new();
        let mut r11_left = SmallVec::new();
        let mut r11_right = SmallVec::new();
        r8_left.push(42);
        r8_right.push(42);
        r8_right.push(8);
        r11_left.push(42);
        r11_left.push(31);
        r11_right.push(42);
        r11_right.push(11);
        r11_right.push(31);
        rules_p2[8] = Rule::Disjunction(true, r8_left, r8_right);
        rules_p2[11] = Rule::Disjunction(true, r11_left, r11_right); */

        rules_p2[0] = Rule::SubRules(
            smallvec![rules_p2.len()],
        );

        for a in 0..6 {
            for b in 0..6 {
                let mut hax = SmallVec::new();
                hax.push(42);
                for _ in 0..a {
                    hax.push(42);
                }
                hax.push(42);
                for _ in 0..b {
                    hax.push(42);
                }
                for _ in 0..b {
                    hax.push(31);
                }
                hax.push(31);

                rules_p2.push(
                    Rule::Disjunction(
                        hax,
                        smallvec![rules_p2.len() + 1],
                    )
                );
            }
        }

        rules_p2.push(Rule::Disjunction(
            smallvec![42],
            smallvec![42, 31],
        ));

        Input{
            rules,
            rules_p2,
            sub_indexes,
            data,
            slices,
        }
    }
}

#[derive(Debug, Clone)]
enum Rule {
    Nop,
    Character(u8),
    SubRules(SmallVec<[usize; 8]>),
    Disjunction(SmallVec<[usize; 8]>, SmallVec<[usize; 8]>),
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: \"a\"
5: \"b\"

ababbb
bababa
abbbab
aaabbb
aaaabbb";

    const EXAMPLE_2: &str = "42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: \"a\"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: \"b\"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&Input::parse(EXAMPLE_1)), 2);
        assert_eq!(part1(&Input::parse(EXAMPLE_2)), 3);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&Input::parse(EXAMPLE_2)), 12);
    }
}