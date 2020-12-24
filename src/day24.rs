use common::aoc::{load_input, print_result, print_time, run_many, run_once};
use smallvec::SmallVec;
use std::ops::{Add, AddAssign};
use rustc_hash::{FxHashSet, FxHashMap};

const W: Coordinate = Coordinate(-2, 0);
const SW: Coordinate = Coordinate(-1, 1);
const SE: Coordinate = Coordinate(1, 1);
const E: Coordinate = Coordinate(2, 0);
const NW: Coordinate = Coordinate(-1, -1);
const NE: Coordinate = Coordinate(1, -1);

const NEIGHBOR_OFFSETS: [Coordinate; 6] = [W, SW, SE, E, NW, NE];

fn main() {
    let (input, dur_load) = run_once(|| load_input("day24"));

    print_time("Load", dur_load);

    let ((list, max_len), dur_parse) = run_many(1000, || parse_input(&input));
    let ((res_part1, coords), dur_part1) = run_many(1000, || part1_old(&list));
    let (res_part2, dur_part2) = run_many(100, || part2(&coords, max_len));

    print_result("P1", res_part1);
    print_result("P2", res_part2);

    print_time("Parse", dur_parse);
    print_time("P1", dur_part1);
    print_time("P2", dur_part2);
    print_time("Total", dur_parse + dur_part1 + dur_part2);
}

fn part1_old(list: &[SmallVec<[Coordinate; 32]>]) -> (usize, Vec<Coordinate>) {
    let mut res = FxHashSet::default();

    for item in list.iter() {
        let path_end = item.iter().fold(Coordinate(0, 0), |acc, x| acc + *x);

        if res.contains(&path_end) {
            res.remove(&path_end);
        } else {
            res.insert(path_end);
        }
    }

    (res.len(), res.into_iter().collect())
}

fn part1(list: &[SmallVec<[Coordinate; 32]>], max_len: usize) -> (usize, Vec<Coordinate>) {
    let width = max_len * 2;
    let mut grid = vec![false; width * width];

    for path in list.iter() {
        let mut path_end = Coordinate(0, 0);
        for offset in path.iter() {
            path_end += *offset;
        }

        let index = path_end.to_index(width);
        grid[index] = !grid[index];
    }

    let res: Vec<Coordinate> = grid.iter()
        .enumerate()
        .filter(|(_, b)| **b)
        .map(|(i, _)| Coordinate::from_index(i, width))
        .collect();

    (res.len(), res)
}

fn part2(coords: &[Coordinate], max_len: usize) -> usize {
    let width = max_len * 2 + 202;
    let mut grid = vec![false; width * width];
    let mut changes = Vec::with_capacity(256);
    for coord in coords.iter() {
        grid[coord.to_index(width)] = true;
    }
    
    let start = width + 1;
    let end = grid.len() - start;

    for _ in 0..100 {
        changes.clear();

        // Flip black tiles and find new whites.
        for (index, is_black) in grid[start..end].iter().enumerate() {
            let index = index + start;

            let coord = Coordinate::from_index(index, width);

            if *is_black {
                let mut count = 0;
                for offset in NEIGHBOR_OFFSETS.iter() {
                    let neigh = coord + *offset;
                    if grid[neigh.to_index(width)] {
                        count += 1;
                        if count > 2 {
                            break;
                        }
                    }
                }

                if count != 1 && count != 2 {
                    changes.push((index, false));
                }
            } else {
                let mut count = 0;
                for offset in NEIGHBOR_OFFSETS.iter() {
                    let neigh = coord + *offset;
                    if grid[neigh.to_index(width)] {
                        count += 1;
                        if count > 2 {
                            break;
                        }
                    }
                }

                if count == 2 {
                    changes.push((index, true));
                }
            }
        }

        for (index, value) in changes.iter() {
            grid[*index] = *value;
        }
    }

    grid.iter().filter(|v| **v).count()
}

fn parse_input(input: &str) -> (Vec<SmallVec<[Coordinate; 32]>>, usize) {
    let res: Vec<SmallVec<[Coordinate; 32]>> = input.lines().filter(|l| l.len() > 0).map(parse_line).collect();
    let max_len = res.iter().map(|r| r.len()).max().unwrap();

    (res, max_len)
}

fn parse_line(line: &str) -> SmallVec<[Coordinate; 32]> {
    let mut res = SmallVec::new();

    let mut modifier = 0u8;
    for c in line.bytes() {
        if c == b's' || c == b'n' {
            modifier = c;
        } else {
            if c == b'w' {
                match modifier {
                    0u8 => res.push(W),
                    b's' => res.push(SW),
                    b'n' => res.push(NW),
                    _ => panic!("Bad stuff")
                }
            } else {
                match modifier {
                    0u8 => res.push(E),
                    b's' => res.push(SE),
                    b'n' => res.push(NE),
                    _ => panic!("Bad stuff")
                }
            }

            modifier = 0u8;
        }
    }

    res
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
struct Coordinate(i32, i32);

impl Coordinate {
    fn to_index(&self, w: usize) -> usize {
        let m = (w / 2) as i32;
        let x = (self.0 + m) as usize;
        let y = (self.1 + m) as usize;

        (y * w) + x
    }

    fn from_index(index: usize, w: usize) -> Coordinate {
        let x = (index % w) as i32;
        let y = (index / w) as i32;
        let m = (w / 2) as i32;

        Coordinate(x - m, y - m)
    }
}

impl Add for Coordinate {
    type Output = Coordinate;

    fn add(self, rhs: Self) -> Self::Output {
        Coordinate(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl AddAssign for Coordinate {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use term::Error::ColorOutOfRange;

    const EXAMPLE_1: &str = "sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew";

    #[test]
    fn test_parse() {
        assert_eq!(parse_line("esenee").as_slice(), &[E, SE, NE, E]);
        assert_eq!(parse_line("esew").as_slice(), &[E, SE, W]);
        assert_eq!(parse_line("nwwswee").as_slice(), &[NW, W, SW, E, E]);
    }

    #[test]
    fn test_old_part() {
        let (input, max_len) = parse_input(EXAMPLE_1);

        let (res_part1, mut coords) = part1(&input, max_len);
        let (res_part1_old, mut coords_old) = part1_old(&input);

        assert_eq!(res_part1, res_part1_old);

        coords.sort();
        coords_old.sort();
        assert_eq!(coords, coords_old);
    }

    #[test]
    fn test_index_convert() {
        assert_eq!(Coordinate(-16, -16).to_index(32), 0);
        assert_eq!(Coordinate(15, 15).to_index(32), (32*32)-1);
        assert_eq!(Coordinate::from_index(0, 32), Coordinate(-16, -16));
        assert_eq!(Coordinate::from_index((32*32)-1, 32), Coordinate(15, 15));

        for w in 44..128 {
            let m = (w / 2) as i32;

            for x in -m..m {
                for y in -m..m {
                    let c = Coordinate(x, y);
                    assert_eq!(Coordinate::from_index(c.to_index(w), w), c);
                }
            }
        }
    }
}
