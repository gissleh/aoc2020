use common::aoc::{load_input, print_result, print_time, run_many, run_once};

const U64_ZERO: u64 = '0' as u64;

fn main() {
    let (input, dur_load) = run_once(|| load_input("day16"));

    print_time("Load", dur_load);

    let (input, dur_parse) = run_many(1000, || Input::parse(&input));
    let ((res_part1, valid_tickets), dur_part1) = run_many(100000, || part1(&input));
    let (res_part2, dur_part2) = run_many(100000, || part2(&input, &valid_tickets));

    print_result("P1", res_part1);
    print_result("P2", res_part2);

    print_time("Parse", dur_parse);
    print_time("P1", dur_part1);
    print_time("P2", dur_part2);
    print_time("Total", dur_parse + dur_part1 + dur_part2);
}

fn part1(input: &Input) -> (u64, Vec<usize>) {
    let mut result = 0;
    let max_value = input.map.len();
    let mut valid_tickets = Vec::with_capacity(input.tickets.len());

    for (i, (ticket_start, ticket_end)) in input.tickets.iter().enumerate() {
        let mut valid = true;
        for n in input.ticket_data[*ticket_start..*ticket_end].iter() {
            let i = *n as usize;

            if i >= max_value || input.map[i] == 0 {
                result += n;
                valid = false;
            }
        }

        if valid {
            valid_tickets.push(i);
        }
    }

    (result, valid_tickets)
}

fn part2(input: &Input, valid_tickets: &[usize]) -> u64 {
    let assigned_fields = determine_fields(input, valid_tickets);
    let mut product = 1;

    for (i, fi) in assigned_fields.iter().enumerate() {
        let rule = input.rules.get(*fi).unwrap();
        if rule.name.starts_with("departure") {
            product *= input.ticket_data[i];
        }
    }

    product
}

fn determine_fields(input: &Input, valid_tickets: &[usize]) -> Vec<usize> {
    let (_, ticket_length) = input.tickets[0];
    let mut field_masks = vec![usize::max_value(); ticket_length];

    for (ticket_start, ticket_end) in valid_tickets.iter().map(|v| &input.tickets[*v]) {
        for (i, n) in input.ticket_data[*ticket_start..*ticket_end].iter().enumerate() {
            field_masks[i] &= input.map[*n as usize]
        }
    }

    let mut field_assigned = vec![false; ticket_length];
    let mut rule_assigned = vec![false; input.rules.len()];
    let mut assigned_fields = vec![0usize; ticket_length];
    let mut assignment_count = 0usize;
    while assignment_count < ticket_length {
        for (i, field_mask) in field_masks.iter().enumerate() {
            if field_assigned[i] {
                continue;
            }

            if field_mask.count_ones() as usize == (assignment_count + 1) {
                for n in 0..input.rules.len() {
                    if rule_assigned[n] {
                        continue;
                    }

                    let n_mask = 1 << n;
                    if field_mask & n_mask == n_mask {
                        field_assigned[i] = true;
                        rule_assigned[n] = true;
                        assigned_fields[i] = n;
                        assignment_count += 1;
                        break
                    }
                }
            }
        }
    }

    return assigned_fields;
}

#[derive(Debug)]
struct Input<'a> {
    map: Vec<usize>,
    rules: Vec<Rule<'a>>,
    ticket_data: Vec<u64>,
    tickets: Vec<(usize, usize)>,
}

impl<'a> Input<'a> {
    fn parse(input: &'a str) -> Input {
        let mut rules = Vec::with_capacity(64);
        let mut ticket_data = Vec::with_capacity(64);
        let mut tickets = Vec::with_capacity(64);

        let mut highest = 0;
        let mut mode = 0;
        for line in input.lines() {
            if line.len() < 2 {
                continue;
            }

            if mode == 0 {
                if line == "your ticket:" {
                    mode = 1;
                    continue;
                }

                let colon_pos = line.find(':').unwrap();
                let or_pos = line.find(" or ").unwrap();

                let rule_text = &line[..colon_pos];
                let (min1, max1) = parse_range(&line[colon_pos+2..or_pos]);
                let (min2, max2) = parse_range(&line[or_pos+4..]);

                rules.push(Rule{
                    name: rule_text,
                    min1, max1,
                    min2, max2,
                });

                if max1 > highest {
                    highest = max1;
                }
                if max2 > highest {
                    highest = max2;
                }
            }

            if mode >= 1 {
                if line == "nearby tickets:" {
                    continue;
                }

                let start = ticket_data.len();
                let mut current = 0u64;
                for c in line.chars() {
                    if c == ',' {
                        ticket_data.push(current);
                        current = 0;
                    } else {
                        current = (current * 10) + (c as u64 - U64_ZERO);
                    }
                }
                ticket_data.push(current);

                tickets.push((start, ticket_data.len()));
            }
        }

        let mut map = vec![0usize; highest as usize + 1];
        for (i, rule) in rules.iter().enumerate() {
            let bit = 1usize << i;

            for n in rule.min1 as usize..=rule.max1 as usize {
                map[n] |= bit;
            }
            for n in rule.min2 as usize..=rule.max2 as usize {
                map[n] |= bit;
            }
        }

        Input{
            map, rules, ticket_data, tickets,
        }
    }
}

fn parse_range(s: &str) -> (u64, u64) {
    let mut res = [0, 0];
    let mut current = 0usize;

    for c in s.chars() {
       if c == '-' {
           current = 1;
       } else {
           res[current] = (res[current] * 10) + (c as u64 - U64_ZERO);
       }
    }

    (res[0], res[1])
}

#[derive(Debug)]
struct Rule<'a> {
    name: &'a str,
    min1: u64,
    max1: u64,
    min2: u64,
    max2: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_part2() {
        let input = Input::parse("class: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9");

        assert_eq!(determine_fields(&input, &[1, 2, 3]), vec![1, 0, 2]);
    }
}