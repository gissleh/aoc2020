use common::aoc::{load_input, run_many, print_time, print_result};
use common::grid::FixedGrid;

const TREE: u8 = '#' as u8;

fn main() {
    let input = load_input("day03");

    let (grid, dur_parse) = run_many(1000, || parse_input(&input));
    let (res_part1, dur_part1) = run_many(100000, || part1(&grid));
    let (res_part2, dur_part2) = run_many(100000, || part2(&grid, res_part1));

    print_result("P1", res_part1);
    print_result("P2", res_part2);

    print_time("Parse", dur_parse);
    print_time("P1", dur_part1);
    print_time("P2", dur_part2);
    print_time("Total", dur_parse + dur_part1 + dur_part2);
}

fn part1(grid: &FixedGrid<u8>) -> usize {
    let width = grid.width();
    let height = grid.height();

    check_slope(grid, 3, width, height)
}

fn part2(grid: &FixedGrid<u8>, res_part1: usize) -> usize {
    let width = grid.width();
    let height = grid.height();

    res_part1
        * check_slope(grid, 1, width, height)
        * check_slope(grid, 5, width, height)
        * check_slope(grid, 7, width, height)
        * check_slope2(grid, 1, 2, width, height)
}

fn check_slope(grid: &FixedGrid<u8>, vx: usize, w: usize, h: usize) -> usize {
    (1..h).filter(|y| grid.get((*y * vx) % w, *y) == TREE).count()
}

fn check_slope2(grid: &FixedGrid<u8>, vx: usize, vy: usize, w: usize, h: usize) -> usize {
    (vy..h).step_by(vy).filter(|y| grid.get(((*y * vx) / vy) % w, *y) == TREE).count()
}

fn parse_input(input: &str) -> FixedGrid<u8> {
    let width = input.find('\n').unwrap();
    let height = input.len() / (width+1);
    let mut data = Vec::with_capacity(width * height);

    for ch in input.chars() {
        if ch == '\n' {
            continue;
        }

        data.push(ch as u8);
    }

    FixedGrid::from(width, height, data)
}

