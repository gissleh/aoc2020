use common::aoc::{load_input_bytes, print_result, print_time, run_many, run_once};
use smallvec::SmallVec;

fn main() {
    let (input, dur_load) = run_once(|| load_input_bytes("day18"));

    print_time("Load", dur_load);

    let (input, dur_parse) = run_many(1000, || Input::parse(&input));
    let (res_part1, dur_part1) = run_many(10000, || part1(&input));
    let (res_part2, dur_part2) = run_many(10000, || part2(&input));

    print_result("P1", res_part1);
    print_result("P2", res_part2);

    print_time("Parse", dur_parse);
    print_time("P1", dur_part1);
    print_time("P2", dur_part2);
    print_time("Total", dur_parse + dur_part1 + dur_part2);
}

fn part1(input: &Input) -> i64 {
    let mut sum = 0;

    for line in input.iter() {
        let (res, _) = solve_one(line);
        sum += res;
    }

    sum
}

fn part2(input: &Input) -> i64 {
    let mut sum = 0;

    for line in input.iter() {
        let (res, _) = solve_one_p2(line);
        sum += res;
    }

    sum
}

fn solve_one(tokens: &[Token]) -> (i64, usize) {
    let mut acc = 0;
    let mut is_adding = true;
    let mut stack: SmallVec<[(i64, bool); 4]> = SmallVec::new();
    let mut pos = 0;
    let mut skip = 0;

    for token in tokens.iter() {
        pos += 1;
        if skip > 0 {
            skip -= 1;
            continue;
        }

        match token {
            Token::Operand(n) => {
                if is_adding {
                    acc += *n;
                } else {
                    acc *= *n;
                }
            }
            Token::Add => {
                is_adding = true;
            }
            Token::Multiply => {
                is_adding = false;
            }
            Token::ParaStart => {
                stack.push((acc, is_adding));
                acc = 0;
                is_adding = true;
            }
            Token::ParaEnd => {
                let (n, was_adding) = stack.pop().unwrap();

                if was_adding {
                    acc += n;
                } else {
                    acc *= n;
                }
            }
        }
    }

    (acc, pos)
}

fn solve_one_p2(tokens: &[Token]) -> (i64, usize) {
    let mut acc = 0;
    let mut pos = 0;
    let mut skip = 0;
    let mut mul_stack: SmallVec<[i64; 16]> = SmallVec::new();

    for token in tokens.iter() {
        pos += 1;
        if skip > 0 {
            skip -= 1;
            continue;
        }

        match token {
            Token::Operand(n) => {
                acc += *n;
            }
            Token::Add => {}
            Token::Multiply => {
                mul_stack.push(acc);
                acc = 0;
            }
            Token::ParaStart => {
                let (n, new_skip) = solve_one_p2(&tokens[pos..]);

                skip = new_skip;
                acc += n;
            }
            Token::ParaEnd => {
                break;
            }
        }
    }

    let prod: i64 = mul_stack.iter().cloned().product();

    if acc > 0 {
        (acc * prod, pos)
    } else {
        (prod, pos)
    }
}

#[derive(Debug)]
struct Input {
    tokens: Vec<Token>,
    lines: Vec<(usize, usize)>,
}

impl Input {
    fn iter(&self) -> impl Iterator<Item = &[Token]> {
        self.lines.iter().map(move |(s, e)| &self.tokens[*s..*e])
    }

    fn parse(input: &[u8]) -> Input {
        let mut tokens = Vec::with_capacity(256);
        let mut lines = Vec::with_capacity(64);
        let mut current = 0;
        let mut start = 0;
        let mut has_number = false;

        for b in input.iter() {
            if has_number && (*b < b'0' || *b > b'9') {
                tokens.push(Token::Operand(current));
                current = 0;
                has_number = false;
            }

            match *b {
                b'\n' => {
                    if start == tokens.len() {
                        continue;
                    }

                    lines.push((start, tokens.len()));
                    start = tokens.len();
                }

                b'+' => {
                    tokens.push(Token::Add);
                }
                b'*' => {
                    tokens.push(Token::Multiply);
                }

                b'(' => {
                    tokens.push(Token::ParaStart);
                }
                b')' => {
                    tokens.push(Token::ParaEnd);
                }

                b'0'..=b'9' => {
                    current += (current * 10) + (*b - b'0') as i64;
                    has_number = true;
                }

                b' ' => {}

                _ => {
                    panic!(format!("Unknown token: {}", *b as char));
                }
            }
        }

        Input { tokens, lines }
    }
}

#[derive(Debug)]
enum Token {
    Operand(i64),
    Add,
    Multiply,
    ParaStart,
    ParaEnd,
}

#[cfg(test)]
pub mod tests {
    use super::*;

    const P2_EXAMPLE: &[u8] = b"1 + (2 * 3) + (4 * (5 + 6))
2 * 3 + (4 * 5)
5 + (8 * 3 + 9 + 3 * 4 * 3)
5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))
((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2
";
    const P2_ANSWERS: &[i64] = &[51, 46, 1445, 669060, 23340];

    #[test]
    fn test_part2() {
        let input = Input::parse(P2_EXAMPLE);

        for (i, line) in input.iter().enumerate() {
            let (res, _) = solve_one_p2(line);
            assert_eq!(res, P2_ANSWERS[i]);
        }
    }
}
