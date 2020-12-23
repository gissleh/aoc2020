#[macro_use]
extern crate smallvec;

use common::aoc::{load_input_bytes, print_result, print_time, run_many, run_once};
use smallvec::SmallVec;

fn main() {
    let (input, dur_load) = run_once(|| load_input_bytes("day23"));

    print_time("Load", dur_load);

    let (game, dur_parse) = run_many(1000, || CupsGame::parse(&input));
    let (res_part1, dur_part1) = run_many(10000, || part1(&game));
    let (res_part2, dur_part2) = run_many(20, || part2(&game));

    print_result("P1", res_part1);
    print_result("P2", res_part2);

    print_time("Parse", dur_parse);
    print_time("P1", dur_part1);
    print_time("P2", dur_part2);
    print_time("Total", dur_parse + dur_part1 + dur_part2);

    assert_eq!(res_part1, 69425837);
}

fn part1(game: &CupsGame) -> usize {
    let mut game = game.clone();

    for _ in 0..100 {
        game.simulate();
    }

    game.print(1) % 100000000
}

fn part2(game: &CupsGame) -> usize {
    let mut game = game.expanded();

    #[cfg(test)]
    {
        println!("{:?}", game);
    }
    for _ in 0..10_000_000 {
        game.simulate();
    }

    let a = game.nexts[1];
    let b = game.nexts[a];

    #[cfg(test)]
    {
        println!("{}", a);
        println!("{}", b);
    }

    assert_eq!(game.count(1), 1000000);

    a * b
}

#[derive(Clone, Debug)]
struct CupsGame {
    current: usize,
    max: usize,
    nexts: SmallVec<[usize; 10]>,
}

impl CupsGame {
    fn expanded(&self) -> CupsGame {
        let mut new_game = CupsGame {
            current: self.current,
            nexts: SmallVec::with_capacity(1000001),
            max: 1000000,
        };

        new_game.nexts.extend_from_slice(&self.nexts);
        let old_last = new_game
            .nexts
            .iter()
            .position(|n| *n == self.current)
            .unwrap();
        new_game.nexts[old_last] = 10;
        while new_game.nexts.len() < 1000000 {
            new_game.nexts.push(new_game.nexts.len() + 1);
        }
        new_game.nexts.push(new_game.current);

        new_game
    }

    fn simulate(&mut self) {
        // Grab 3 cups
        let a = self.nexts[self.current];
        let b = self.nexts[a];
        let c = self.nexts[b];

        // Take them out.
        self.nexts[self.current] = self.nexts[c];
        self.nexts[c] = a;

        // Find current.
        let mut destination = if self.current > 1 {
            self.current - 1
        } else {
            self.max
        };
        while a == destination || b == destination || c == destination {
            destination -= 1;
            if destination == 0 {
                destination = self.max;
            }
        }

        // Put them back.
        self.nexts[c] = self.nexts[destination];
        self.nexts[destination] = a;
        self.current = self.nexts[self.current];
    }

    fn print(&self, from: usize) -> usize {
        let mut sum = from;
        let mut current = self.nexts[from];

        while current != from {
            sum = sum * 10 + current;
            current = self.nexts[current];
        }

        sum
    }

    fn count(&self, from: usize) -> usize {
        let mut count = 1;
        let mut current = self.nexts[from];

        while current != from {
            count += 1;
            current = self.nexts[current];

            if count > self.max {
                panic!("BAD LOOP");
            }
        }

        count
    }

    fn parse(a: &[u8]) -> CupsGame {
        let start = (a[0] - b'0') as usize;
        let mut current = start;
        let mut nexts = smallvec![0usize; 10];

        for next in a.iter().skip(1) {
            let next = (*next - b'0') as usize;
            if next > 9 {
                nexts[current] = start;
                break;
            }

            nexts[current] = next;
            current = next;
        }

        CupsGame {
            current: start,
            max: 9,
            nexts,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&CupsGame::parse("389125467\n".as_bytes())), 67384529);
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(&CupsGame::parse("389125467\n".as_bytes())),
            149245887792
        );
    }
}
