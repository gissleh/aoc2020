use common::aoc::{print_result, print_time, run_many, run_once, load_input_bytes};
use common::grid::FixedGrid;

const C_SLASH: u8 = '/' as u8;
const C_SPACE: u8 = ' ' as u8;
const C_HASH: u8 = '#' as u8;
const C_DOT: u8 = '.' as u8;
const C_NEWLINE: u8 = '\n' as u8;

fn main() {
    let (input, dur_load) = run_once(|| load_input_bytes("year2017-day21"));

    print_time("Load", dur_load);

    let (rules, dur_parse) = run_many(1000, || parse_input(&input));
    let (res_part1, dur_part1) = run_many(100, || part1(&rules, 5));
    let (res_part2, dur_part2) = run_many(10, || part1(&rules, 18));

    print_result("P1", res_part1);
    print_result("P2", res_part2);

    print_time("Parse", dur_parse);
    print_time("P1", dur_part1);
    print_time("P2", dur_part2);
    print_time("Total", dur_parse + dur_part1 + dur_part2);
}

fn part1(rules: &[Rule], iterations: usize) -> usize {
    let mut grid_width = 3;
    for _ in 0..iterations {
        let key_width = if grid_width & 1 == 1 {3} else {2};
        grid_width = (key_width + 1) * (grid_width / key_width)
    }

    let mut width = 3;
    let mut grid = FixedGrid::new(grid_width, grid_width, C_DOT);
    let mut keys = Vec::with_capacity(32);
    grid.set(1, 0, C_HASH);
    grid.set(2, 1, C_HASH);
    grid.set(0, 2, C_HASH);
    grid.set(1, 2, C_HASH);
    grid.set(2, 2, C_HASH);

    for _ in 0..iterations {
        let key_width = if width & 1 == 1 {3} else {2};
        let result_width = key_width + 1;
        let chunk_width = width / key_width;

        keys.clear();
        for iy in 0..chunk_width {
            for ix in 0..chunk_width {
                let x = ix * key_width;
                let y = iy * key_width;

                keys.push(RuleKey::from_grid(&grid, x, y, key_width));
            }
        }

        for (i, key) in keys.iter().enumerate() {
            let iy = i / chunk_width;
            let ix = i % chunk_width;
            let x = ix * result_width;
            let y = iy * result_width;

            for rule in rules.iter() {
                if let Some(_) = rule.keys.iter().position(|k| *k == *key) {
                    rule.result.to_grid(&mut grid, x, y);
                    break;
                }
            }
        }

        width = result_width * chunk_width
    }

    grid.count(C_HASH)
}

fn parse_input(input: &[u8]) -> Vec<Rule> {
    let mut rules = Vec::with_capacity(64);

    for line in input.split(|c| *c == C_NEWLINE) {
        if line.len() == 0 {
            continue;
        }

        let pos = line.iter().position(|b| *b == C_SPACE).unwrap();
        let key = RuleKey::from_input(&line[..pos]);
        let r1 = key.rotated();
        let r2 = r1.rotated();
        let r3 = r2.rotated();

        let mut keys = vec![
            key.flipped_h(),
            key.flipped_v(),
            key,
            r1.flipped_h(),
            r1.flipped_v(),
            r1,
            r2.flipped_h(),
            r2.flipped_v(),
            r2,
            r3.flipped_h(),
            r3.flipped_v(),
            r3,
        ];

        let mut dup_list = Vec::new();
        'duploop: for (i, k) in keys.iter().enumerate() {
            for k2 in keys[i+1..].iter() {
                if k == k2 {
                    dup_list.push(i - dup_list.len());
                    continue 'duploop;
                }
            }
        }
        for dup in dup_list {
            keys.remove(dup);
        }

        rules.push( Rule{
            keys, result: RuleKey::from_input(&line[pos+4..])
        });
    }

    rules
}

struct Rule {
    keys: Vec<RuleKey>,
    result: RuleKey,
}

#[derive(PartialEq, Clone)]
struct RuleKey {
    width: usize,
    pattern: Vec<u8>,
}

impl RuleKey {
    fn flipped_h(&self) -> RuleKey {
        let mut copy = self.clone();
        copy.flip_h();

        copy
    }

    fn flip_h(&mut self) {
        if self.width == 2 {
            self.pattern.swap(0, 1);
            self.pattern.swap(2, 3);
        } else {
            self.pattern.swap(0, 2);
            self.pattern.swap(3, 5);
            self.pattern.swap(6, 8);
        }
    }

    fn flipped_v(&self) -> RuleKey {
        let mut copy = self.clone();
        copy.flip_v();

        copy
    }

    fn flip_v(&mut self) {
        if self.width == 2 {
            self.pattern.swap(0, 2);
            self.pattern.swap(1, 3);
        } else {
            self.pattern.swap(0, 6);
            self.pattern.swap(1, 7);
            self.pattern.swap(2, 8);
        }
    }

    fn rotated(&self) -> RuleKey {
        let mut copy = self.clone();
        copy.rotate();

        copy
    }

    fn rotate(&mut self) {
        if self.width == 2 {
            let p0 = self.pattern[0];
            let p1 = self.pattern[1];
            let p2 = self.pattern[2];
            let p3 = self.pattern[3];

            self.pattern[0] = p2;
            self.pattern[1] = p0;
            self.pattern[2] = p3;
            self.pattern[3] = p1;
        } else {
            let p0 = self.pattern[0];
            let p1 = self.pattern[1];
            let p2 = self.pattern[2];
            let p3 = self.pattern[3];
            let p4 = self.pattern[4];
            let p5 = self.pattern[5];
            let p6 = self.pattern[6];
            let p7 = self.pattern[7];
            let p8 = self.pattern[8];

            self.pattern[0] = p6;
            self.pattern[1] = p3;
            self.pattern[2] = p0;
            self.pattern[3] = p7;
            self.pattern[4] = p4;
            self.pattern[5] = p1;
            self.pattern[6] = p8;
            self.pattern[7] = p5;
            self.pattern[8] = p2;
        }
    }

    fn to_grid(&self, grid: &mut FixedGrid<u8>, x: usize, y: usize) {
        for (i, v) in self.pattern.iter().enumerate() {
            let x = x + (i % self.width);
            let y = y + (i / self.width);

            grid.set(x, y, *v);
        }
    }

    fn from_input(input: &[u8]) -> RuleKey {
        let mut width = 0;
        let mut pattern = Vec::with_capacity(9);

        for c in input.iter() {
            match *c {
                C_SLASH => {
                    if width == 0 {
                        width = pattern.len();
                    }
                }
                _ => {
                    pattern.push(*c);
                }
            }
        }

        RuleKey{width, pattern}
    }

    fn from_grid(grid: &FixedGrid<u8>, x: usize, y: usize, width: usize) -> RuleKey {
        let mut pattern = Vec::with_capacity(width * width);

        for y in y..y+width {
            for x in x..x+width {
                pattern.push(grid.get(x, y));
            }
        }

        RuleKey{width, pattern}
    }
}