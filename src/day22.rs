use common::aoc::{load_input, print_result, print_time, run_many, run_once};
use smallvec::SmallVec;
use common::parsers::{parse_u8};
use smallvec::alloc::collections::VecDeque;
use rustc_hash::{FxHashSet, FxHashMap, FxHasher};
use std::collections::HashMap;
use std::hash::BuildHasherDefault;

fn main() {
    let (input, dur_load) = run_once(|| load_input("day22"));

    print_time("Load", dur_load);

    let ((deck1, deck2), dur_parse) = run_many(1000, || parse_input(&input));
    let (res_part1, dur_part1) = run_many(100000, || part1(&deck1, &deck2));
    let (res_part2, dur_part2) = run_many(100, || part2(&deck1, &deck2));

    print_result("P1", res_part1);
    print_result("P2", res_part2);

    print_time("Parse", dur_parse);
    print_time("P1", dur_part1);
    print_time("P2", dur_part2);
    print_time("Total", dur_parse + dur_part1 + dur_part2);

    assert_eq!(res_part1, 33400);
    assert_eq!(res_part2, 33745);
}

fn part1(deck_1: &[u8], deck_2: &[u8]) -> u32 {
    let mut deck_1 = VecDeque::from(Vec::from(deck_1));
    let mut deck_2 = VecDeque::from(Vec::from(deck_2));

    while !deck_1.is_empty() && !deck_2.is_empty() {
        let card_1 = deck_1.pop_front().unwrap();
        let card_2 = deck_2.pop_front().unwrap();

        if card_1 > card_2 {
            deck_1.push_back(card_1);
            deck_1.push_back(card_2);
        } else {
            deck_2.push_back(card_2);
            deck_2.push_back(card_1);
        }
    }

    let winner = if deck_1.is_empty() {
        deck_2
    } else {
        deck_1
    };

    let l = winner.len();
    winner.iter().enumerate().map(|(i, c)| (l - i) as u32 * *c as u32).sum()
}

fn part2(deck_1: &[u8], deck_2: &[u8]) -> u32 {
    let mut cache = FxHashMap::default();

    let (score, _) = part2_recurse(
        VecDeque::from(Vec::from(deck_1)),
        VecDeque::from(Vec::from(deck_2)),
        &mut cache,
    );

    score
}

fn part2_recurse(mut deck_1: VecDeque<u8>, mut deck_2: VecDeque<u8>, mut cache: &mut HashMap<SmallVec<[u8; 64]>, u32, BuildHasherDefault<FxHasher>>) -> (u32, u32) {
    let mut set = FxHashSet::default();
    let mut override_winner = false;

    #[cfg(test)]
    println!("GAME {:?}", deck_fingerprint(&deck_1, &deck_2));

    while !deck_1.is_empty() && !deck_2.is_empty() {
        let print = deck_fingerprint(&deck_1, &deck_2);
        if !set.insert(print) {
            override_winner = true;
            break;
        }

        let card_1 = deck_1.pop_front().unwrap();
        let card_2 = deck_2.pop_front().unwrap();

        if deck_1.len() >= card_1 as usize && deck_2.len() >= card_2 as usize {
            let mut sub_deck_1 = deck_1.clone();
            while sub_deck_1.len() > card_1 as usize {
                sub_deck_1.pop_back();
            }
            let mut sub_deck_2 = deck_2.clone();
            while sub_deck_2.len() > card_2 as usize {
                sub_deck_2.pop_back();
            }

            let print = deck_fingerprint(&sub_deck_1, &sub_deck_2);

            let sub_winner = if let Some(result) = cache.get(&print) {
                *result
            } else {
                let sub_winner = part2_recurse_inner(sub_deck_1, sub_deck_2, &mut cache);
                cache.insert(print, sub_winner);
                sub_winner
            };

            if sub_winner == 1 {
                deck_1.push_back(card_1);
                deck_1.push_back(card_2);
            } else {
                deck_2.push_back(card_2);
                deck_2.push_back(card_1);
            }
        } else {
            if card_1 > card_2 {
                deck_1.push_back(card_1);
                deck_1.push_back(card_2);
            } else {
                deck_2.push_back(card_2);
                deck_2.push_back(card_1);
            }
        }
    }

    let winner_player;
    let winner = if override_winner || deck_2.is_empty() {
        winner_player = 1;
        deck_1
    } else {
        winner_player = 2;
        deck_2
    };

    let l = winner.len();

    (
        winner.iter().enumerate().map(|(i, c)| (l - i) as u32 * *c as u32).sum(),
        winner_player,
    )
}

fn part2_recurse_inner(deck_1: VecDeque<u8>, deck_2: VecDeque<u8>, mut cache: &mut HashMap<SmallVec<[u8; 64]>, u32, BuildHasherDefault<FxHasher>>) -> u32 {
    let mut highest = 0;
    let mut highest_player = 1;

    for n in deck_1.iter() {
        if *n > highest {
            highest_player = 1;
            highest = *n;
        }
    }
    for n in deck_2.iter() {
        if *n > highest {
            highest_player = 2;
            highest = *n;
        }
    }

    if highest_player == 1 {
        highest_player
    } else {
        let (_, highest_player) = part2_recurse(deck_1, deck_2, &mut cache);
        highest_player
    }
}

fn deck_fingerprint(deck_1: &VecDeque<u8>, deck_2: &VecDeque<u8>) -> SmallVec<[u8; 64]> {
    let mut res = SmallVec::with_capacity(deck_1.len() + deck_2.len() + 1);

    res.extend(deck_1.iter().cloned());
    res.push(255);
    res.extend(deck_2.iter().cloned());

    res
}

fn parse_input(input: &str) -> (SmallVec<[u8; 64]>, SmallVec<[u8; 64]>) {
    let mut deck_1 = SmallVec::default();
    let mut deck_2 = SmallVec::default();
    let mut player = 1;

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        if line.len() < 5 {
            if player == 2 {
                deck_2.push(parse_u8(line));
            } else {
                deck_1.push(parse_u8(line));
            }
        } else if line.starts_with("Player ") {
            player = (line.as_bytes()[7] - b'0') as usize;
        }
    }

    (deck_1, deck_2)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10";

    const EXAMPLE_2: &str = "Player 1:
43
19

Player 2:
2
29
14";

    #[test]
    fn test_part1() {
        let (deck_1, deck_2) = parse_input(EXAMPLE_1);

        assert_eq!(part1(&deck_1, &deck_2), 306);
    }


    #[test]
    fn test_part2() {
        let (deck_1, deck_2) = parse_input(EXAMPLE_1);

        assert_eq!(part2(&deck_1, &deck_2), 291);

        let (deck_1, deck_2) = parse_input(EXAMPLE_2);

        assert_eq!(part2(&deck_1, &deck_2), 105);
    }
}