use common::aoc::{load_input_bytes, print_result, print_time, run_many, run_once};

const NEWLINE: u8 = '\n' as u8;
const SEAT_VACANT: u8 = 'L' as u8;
const SEAT_OCCUPIED: u8 = '#' as u8;
const EDGE: u8 = 'E' as u8;

fn main() {
    let (input, dur_load) = run_once(|| load_input_bytes("day11"));

    print_time("Load", dur_load);

    let ((grid, width), dur_parse) = run_many(1000, || parse_input(&input));
    let (res_part1, dur_part1) = run_many(1000, || part1(&grid, width));
    let (res_part2, dur_part2) = run_many(1000, || part2(&grid, width));

    print_result("P1", res_part1);
    print_result("P2", res_part2);

    print_time("Parse", dur_parse);
    print_time("P1", dur_part1);
    print_time("P2", dur_part2);
    print_time("Total", dur_parse + dur_part1 + dur_part2);
}

fn part1(grid: &[u8], width: usize) -> usize {
    let mut current = Vec::from(grid);
    let start = width + 1;
    let end = grid.len() - width;
    let positions = (start..end)
        .filter(|n| grid[*n] == SEAT_VACANT)
        .collect::<Vec<usize>>();
    let tl_offset = usize::max_value() - width;
    let offsets = [
        tl_offset,
        tl_offset + 1,
        tl_offset + 2,
        usize::max_value(),
        1,
        width - 1,
        width,
        width + 1,
    ];

    let mut vacate = Vec::with_capacity(positions.len());
    let mut occupy = Vec::with_capacity(positions.len());

    loop {
        'pos_loop: for pos in positions.iter() {
            if current[*pos] == SEAT_OCCUPIED {
                let mut count = 0;
                for offset in offsets.iter() {
                    if current[pos + offset] == SEAT_OCCUPIED {
                        count += 1;
                        if count == 4 {
                            vacate.push(*pos);
                            continue 'pos_loop;
                        }
                    }
                }
            } else {
                for offset in offsets.iter() {
                    if current[pos + offset] == SEAT_OCCUPIED {
                        continue 'pos_loop;
                    }
                }

                occupy.push(*pos);
            }
        }

        if occupy.len() == 0 && vacate.len() == 0 {
            return current.iter().filter(|c| **c == SEAT_OCCUPIED).count();
        }

        for pos in occupy.iter() {
            current[*pos] = SEAT_OCCUPIED;
        }
        for pos in vacate.iter() {
            current[*pos] = SEAT_VACANT;
        }
        vacate.clear();
        occupy.clear();
    }
}

fn part2(grid: &[u8], width: usize) -> usize {
    let mut current = Vec::from(grid);
    let start = width + 1;
    let end = grid.len() - width;
    let positions = (start..end)
        .filter(|n| grid[*n] == SEAT_VACANT)
        .collect::<Vec<usize>>();
    let tl_offset = usize::max_value() - width;
    let offsets = [
        tl_offset,
        tl_offset + 1,
        tl_offset + 2,
        usize::max_value(),
        1,
        width - 1,
        width,
        width + 1,
    ];

    let mut vacate = Vec::with_capacity(positions.len());
    let mut occupy = Vec::with_capacity(positions.len());

    loop {
        'pos_loop: for pos in positions.iter() {
            if current[*pos] == SEAT_OCCUPIED {
                let mut count = 0;
                for offset in offsets.iter() {
                    for n in 1.. {
                        let v = current[pos + (n * offset)];
                        if v == SEAT_OCCUPIED {
                            count += 1;
                            if count == 5 {
                                vacate.push(*pos);
                                continue 'pos_loop;
                            }

                            break;
                        } else if v == EDGE || v == SEAT_VACANT {
                            break;
                        }
                    }
                }
            } else {
                for offset in offsets.iter() {
                    for n in 1.. {
                        let v = current[pos + (n * offset)];
                        if v == SEAT_OCCUPIED {
                            continue 'pos_loop;
                        } else if v == EDGE || v == SEAT_VACANT {
                            break;
                        }
                    }
                }

                occupy.push(*pos);
            }
        }

        if occupy.len() == 0 && vacate.len() == 0 {
            return current.iter().filter(|c| **c == SEAT_OCCUPIED).count();
        }

        for pos in occupy.iter() {
            current[*pos] = SEAT_OCCUPIED;
        }
        for pos in vacate.iter() {
            current[*pos] = SEAT_VACANT;
        }
        vacate.clear();
        occupy.clear();
    }
}

fn parse_input(input: &[u8]) -> (Vec<u8>, usize) {
    let width = input.iter().position(|b| *b == NEWLINE).unwrap();
    let height = input.len() / (width + 1);

    let mut res = vec![EDGE; (width + 2) * (height + 2)];

    for i in 0..height {
        let dst_pos = ((i + 1) * (width + 2)) + 1;
        let dst = &mut res[dst_pos..dst_pos + width];
        let src = &input[(i * (width + 1))..(((i + 1) * (width + 1)) - 1)];

        dst.copy_from_slice(src);
    }

    (res, width + 2)
}
