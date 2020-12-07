use common::aoc::{load_input, print_result, print_time, run_many, run_once};
use std::collections::BTreeMap;

const ZERO: u32 = '0' as u32;

fn main() {
    let (input, dur_load) = run_once(|| load_input("day07"));

    print_time("Load", dur_load);

    let (rule_set, dur_parse) = run_many(1000, || RuleSet::parse(&input));
    let (res_part1, dur_part1) = run_many(100000, || rule_set.count_containers("shiny gold"));
    let (res_part2, dur_part2) = run_many(100000, || rule_set.count_bags("shiny gold"));

    print_result("P1", res_part1);
    print_result("P2", res_part2);

    print_time("Parse", dur_parse);
    print_time("P1", dur_part1);
    print_time("P2", dur_part2);
    print_time("Total", dur_parse + dur_part1 + dur_part2);
}

struct RuleSet {
    map: BTreeMap<String, usize>,
    bags: Vec<Bag>,
}

impl RuleSet {
    pub fn count_containers(&self, name: &str) -> u32 {
        let bag = self.bag(name).unwrap();
        let mut explored = vec![false; self.bags.len()];
        let mut stack = Vec::with_capacity(64);
        let mut count = 0;

        explored[bag.index] = true;
        stack.push(bag);

        while stack.len() > 0 {
            let bag = stack.pop().unwrap();

            for BagLink(index, _) in bag.contained_by.iter() {
                if !explored[*index] {
                    explored[*index] = true;
                    count += 1;
                    stack.push(self.bags.get(*index).unwrap());
                }
            }
        }

        count
    }

    pub fn count_bags(&self, name: &str) -> u32 {
        let bag = self.bag(name).unwrap();
        let mut explored = vec![false; self.bags.len()];
        let mut stack = Vec::with_capacity(64);
        let mut count = 0;

        explored[bag.index] = true;
        stack.push((bag, 1u32));

        while stack.len() > 0 {
            let (bag, c) = stack.pop().unwrap();
            for BagLink(index, count2) in bag.can_contain.iter() {
                count += c * *count2;
                stack.push((
                    self.bags.get(*index).unwrap(),
                    c * *count2,
                ));
            }
        }

        count
    }

    fn bag(&self, s: &str) -> Option<&Bag> {
        self.map.get(s).map(|f| self.bags.get(*f).unwrap())
    }

    fn ensure_bag_index(&mut self, s: &str) -> usize {
        match self.map.get(s) {
            Some(index) => *index,
            None => {
                let index = self.bags.len();
                let bag = Bag{
                    index,
                    can_contain: Vec::with_capacity(4),
                    contained_by: Vec::with_capacity(8),
                };

                self.map.insert(String::from(s), index);
                self.bags.push(bag);

                index
            }
        }
    }

    pub fn parse(input: &str) -> RuleSet {
        let mut rule_set = RuleSet{
            map: BTreeMap::new(),
            bags: Vec::with_capacity(256),
        };

        for line in input.lines() {
            let bag_color_index = find_nth(line, ' ', 2);
            let bag_index = rule_set.ensure_bag_index(&line[..bag_color_index]);
            let offset = bag_color_index + 13;

            for part in line[offset..].split(",") {
                let count = (part.chars().skip(1).next().unwrap() as u32) - ZERO;
                if count > 9 {
                    continue
                }

                let child_bag_color_index = find_nth(part, ' ', 4);
                let child_bag_color = &part[3..child_bag_color_index];
                let child_bag_index = rule_set.ensure_bag_index(child_bag_color);

                rule_set.bags[bag_index].can_contain.push(BagLink(child_bag_index, count));
                rule_set.bags[child_bag_index].contained_by.push(BagLink(bag_index, count));
            }
        }

        rule_set
    }
}

#[derive(Debug)]
struct Bag {
    index: usize,
    can_contain: Vec<BagLink>,
    contained_by: Vec<BagLink>,
}

#[derive(Debug)]
struct BagLink (usize, u32);

fn find_nth(s: &str, c: char, n: usize) -> usize {
    let mut n = n;
    for (i, c2) in s.chars().enumerate() {
        if c2 == c {
            n -= 1;
            if n == 0 {
                return i;
            }
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let rs1 = RuleSet::parse("light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.");
        let rs2 = RuleSet::parse("shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.");

        assert_eq!(rs2.count_bags("shiny gold"), 126);
        assert_eq!(rs1.count_bags("shiny gold"), 32);
    }
}
