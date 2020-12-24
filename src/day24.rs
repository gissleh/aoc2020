use common::aoc::{load_input, print_result, print_time, run_many, run_once};
use smallvec::SmallVec;
use std::ops::{Add, AddAssign};
use common::grid::FixedGrid;

const W: Coordinate = Coordinate(-1, 0);
const SW: Coordinate = Coordinate(-1, 1);
const SE: Coordinate = Coordinate(0, 1);
const E: Coordinate = Coordinate(1, 0);
const NW: Coordinate = Coordinate(0, -1);
const NE: Coordinate = Coordinate(1, -1);

const NEIGHBOR_OFFSETS: [(usize, usize); 6] = [
    (W.0 as usize, W.1 as usize),
    (SW.0 as usize, SW.1 as usize),
    (SE.0 as usize, SE.1 as usize),
    (E.0 as usize, E.1 as usize),
    (NW.0 as usize, NW.1 as usize),
    (NE.0 as usize, NE.1 as usize),
];

fn main() {
    let (input, dur_load) = run_once(|| load_input("day24"));

    print_time("Load", dur_load);

    let ((list, max_len), dur_parse) = run_many(1000, || parse_input(&input));
    let ((res_part1, coords), dur_part1) = run_many(1000, || part1(&list, max_len));
    let (res_part2, dur_part2) = run_many(100, || part2(&coords, max_len));

    print_result("P1", res_part1);
    print_result("P2", res_part2);

    print_time("Parse", dur_parse);
    print_time("P1", dur_part1);
    print_time("P2", dur_part2);
    print_time("Total", dur_parse + dur_part1 + dur_part2);
}

fn part1(list: &[SmallVec<[Coordinate; 64]>], max_len: usize) -> (usize, Vec<Coordinate>) {
    let mut grid = vec![vec![false; max_len*2]; max_len*2];
    let center = max_len as i32;

    for item in list.iter() {
        let Coordinate(x, y) = item.iter().fold(Coordinate(0, 0), |acc, x| acc + *x);

        let x = (x + center) as usize;
        let y = (y + center) as usize;

        let v = &mut grid[y][x];
        *v = !*v;
    }

    let mut res = Vec::with_capacity(512);
    for (y, row) in grid.iter().enumerate() {
        for (x, col) in row.iter().enumerate() {
            if *col {
                res.push(Coordinate((x+100) as i32, (y+100) as i32))
            }
        }
    }

    (res.len(), res.into_iter().collect())
}

fn part2(coords: &[Coordinate], max_len: usize) -> usize {
    let mut grid = FixedGrid::new((max_len + 100) * 2, (max_len + 100) * 2, false);
    let mut changes = Vec::with_capacity(128);
    let mut fx = grid.width() / 2;
    let mut fy = grid.height() / 2;
    let mut tx = fx;
    let mut ty = fy;

    for Coordinate(x, y) in coords.iter() {
        let x = *x as usize;
        let y = *y as usize;
        if x <= fx {
            fx = x - 1;
        } else if x >= tx {
            tx = x + 1;
        }
        if y <= fy {
            fy = y - 1;
        } else if y >= ty {
            ty = y + 1;
        }

        grid.set(x, y, true);
    }

    for _ in 0..100 {
        changes.clear();

        fx -= 1;
        fy -= 1;
        tx += 1;
        ty += 1;

        // Flip black tiles and find new whites.
        for (x, y, is_black) in grid.limited_iter(fx, fy, tx, ty) {
            let mut count = 0;
            for (offset_x, offset_y) in NEIGHBOR_OFFSETS.iter() {
                if grid.get(x+offset_x, y+offset_y) {
                    count += 1;
                    if count > 2 {
                        break;
                    }
                }
            }

            if *is_black {
                if count != 1 && count != 2 {
                    changes.push(((x, y), false));
                }
            } else {
                if count == 2 {
                    changes.push(((x, y), true));
                }
            }
        }

        for ((x, y), value) in changes.iter() {
            grid.set(*x, *y, *value);
        }
    }

    grid.data().iter().filter(|v| **v).count()
}

fn parse_input(input: &str) -> (Vec<SmallVec<[Coordinate; 64]>>, usize) {
    let res: Vec<SmallVec<[Coordinate; 64]>> = input.lines().filter(|l| l.len() > 0).map(parse_line).collect();
    let max_len = res.iter().map(|v| v.len()).max().unwrap();

    (res, max_len)
}

fn parse_line(line: &str) -> SmallVec<[Coordinate; 64]> {
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

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Coordinate(i32, i32);

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

    #[test]
    fn test_parse() {
        assert_eq!(parse_line("esenee").as_slice(), &[E, SE, NE, E]);
        assert_eq!(parse_line("esew").as_slice(), &[E, SE, W]);
        assert_eq!(parse_line("nwwswee").as_slice(), &[NW, W, SW, E, E]);
    }
}
