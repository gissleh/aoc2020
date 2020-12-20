#[macro_use]
extern crate smallvec;

use common::aoc::{load_input, print_result, print_time, run_many, run_once};
use smallvec::SmallVec;
use common::parsers::parse_u64;
use common::grid::FixedGrid;
use chrono::format::Item::Fixed;
use termion::event::Key::Right;

const RIGHT: usize = 0;
const DOWN: usize = 1;
const LEFT: usize = 2;
const UP: usize = 3;
const DIRECTIONS: [usize; 4]= [RIGHT, DOWN, LEFT, UP];
const FLIPPED_H: [usize; 4] = [LEFT, UP, RIGHT, DOWN];
const FLIPPED_V: [usize; 4] = [RIGHT, UP, LEFT, DOWN];
const FLIPPED_VH: [usize; 4] = [LEFT, UP, RIGHT, DOWN];

const TILE_INDEX_ROTATED: [[usize; 100]; 3] = [
    [
        90, 80, 70, 60, 50, 40, 30, 20, 10,  0,
        91, 81, 71, 61, 51, 41, 31, 21, 11,  1,
        92, 82, 72, 62, 52, 42, 32, 22, 12,  2,
        93, 83, 73, 63, 53, 43, 33, 23, 13,  3,
        94, 84, 74, 64, 54, 44, 34, 24, 14,  4,
        95, 85, 75, 65, 55, 45, 35, 25, 15,  5,
        96, 86, 76, 66, 56, 46, 36, 26, 16,  6,
        97, 87, 77, 67, 57, 47, 37, 27, 17,  7,
        98, 88, 78, 68, 58, 48, 38, 28, 18,  8,
        99, 89, 79, 69, 59, 49, 39, 29, 19,  9
    ],
    [
        99, 98, 97, 96, 95, 94, 93, 92, 91, 90,
        89, 88, 87, 86, 85, 84, 83, 82, 81, 80,
        79, 78, 77, 76, 75, 74, 73, 72, 71, 70,
        69, 68, 67, 66, 65, 64, 63, 62, 61, 60,
        59, 58, 57, 56, 55, 54, 53, 52, 51, 50,
        49, 48, 47, 46, 45, 44, 43, 42, 41, 40,
        39, 38, 37, 36, 35, 34, 33, 32, 31, 30,
        29, 28, 27, 26, 25, 24, 23, 22, 21, 20,
        19, 18, 17, 16, 15, 14, 13, 12, 11, 10,
        9,  8,  7,  6,  5,  4, 3,  2,  1,  0
    ],
    [
        9, 19, 29, 39, 49, 59, 69, 79, 89, 99,
        8, 18, 28, 38, 48, 58, 68, 78, 88, 98,
        7, 17, 27, 37, 47, 57, 67, 77, 87, 97,
        6, 16, 26, 36, 46, 56, 66, 76, 86, 96,
        5, 15, 25, 35, 45, 55, 65, 75, 85, 95,
        4, 14, 24, 34, 44, 54, 64, 74, 84, 94,
        3, 13, 23, 33, 43, 53, 63, 73, 83, 93,
        2, 12, 22, 32, 42, 52, 62, 72, 82, 92,
        1, 11, 21, 31, 41, 51, 61, 71, 81, 91,
        0, 10, 20, 30, 40, 50, 60, 70, 80, 90,
    ],
];

fn main() {
    let (input, dur_load) = run_once(|| load_input("day20"));

    print_time("Load", dur_load);

    let (tiles, dur_parse) = run_many(1000, || parse_input(&input));

    let ((res_part1, states), dur_part1) = run_many(1, || part1(&tiles));
    let (assembled_image, dur_assemble) = run_many(1, || assemble_image(&states));
    let (res_part2, dur_part2) = run_many(1000, || part2(&assembled_image));

    print_result("P1", res_part1);
    print_result("P2", res_part2);

    print_time("Parse", dur_parse);
    print_time("P1", dur_part1);
    print_time("P2 Assemble", dur_assemble);
    print_time("P2 Search", dur_part2);
    print_time("Total", dur_parse + dur_part1 + dur_assemble + dur_part2);
}

fn pos_dir(x: usize, y: usize, dir: usize) -> (usize, usize) {
    match dir {
        RIGHT => (x + 10, y),
        DOWN => (x, y + 10),
        LEFT => (x - 10, y),
        UP => (x, y - 10),
        _ => panic!("INVALID DIRECTION"),
    }
}

fn part1(tiles: &[Tile]) -> (u64, Vec<TileState>) {
    let mut states: Vec<TileState> = tiles.iter().map(|t| TileState::from(t)).collect();
    let mut placed = vec![false; tiles.len()];
    let mut stack: SmallVec<[usize; 16]> = smallvec![0];

    while stack.len() > 0 {
        let index = stack.pop().unwrap();

        placed[index] = true;

        'connect_loop: for dir in 0..4 {
            if states[index].connected[dir] {
                continue;
            }

            let horizontal = dir == LEFT || dir == RIGHT;
            let mut target_dir = FLIPPED_VH[dir];
            let mut side = states[index].tile.sides[dir];

            for other_index in 0..states.len() {
                if placed[other_index] {
                    continue;
                }

                for _ in 0..4 {
                    let mut good = side == states[other_index].tile.sides[target_dir];
                    if !good {
                        states[other_index].tile.flip_v();
                        good = side == states[other_index].tile.sides[target_dir]
                    }
                    if !good {
                        states[other_index].tile.flip_v();
                        states[other_index].tile.flip_h();
                        good = side == states[other_index].tile.sides[target_dir]
                    }
                    if !good {
                        states[other_index].tile.flip_h();
                    }

                    if good {
                        states[index].connected[dir] = true;
                        states[index].connections[dir] = other_index;
                        states[index].connected_count += 1;
                        states[other_index].connected[target_dir] = true;
                        states[other_index].connections[target_dir] = index;
                        states[other_index].connected_count += 1;

                        stack.push(other_index);

                        continue 'connect_loop;
                    }

                    states[other_index].tile.rotate(1);
                }

                /* There's a better but broken solution below, but I've had enough of this puzzle.
                for other_dir in 0..4 {
                    let match_unflipped = side == states[other_index].tile.sides[other_dir];
                    let match_flipped = side == states[other_index].tile.flipped_sides[other_dir];
                    let match_any = match_flipped || match_unflipped;

                    if match_any {
                        if other_dir != target_dir {
                            states[other_index].tile.rotate((4 - other_dir + target_dir) % 4);
                        }
                        if match_flipped {
                            if horizontal {
                                states[other_index].tile.flip_v();
                            } else {
                                states[other_index].tile.flip_h();
                            }
                        }

                        states[index].connected[dir] = true;
                        states[index].connections[dir] = other_index;
                        states[index].connected_count += 1;
                        states[other_index].connected[target_dir] = true;
                        states[other_index].connections[target_dir] = index;
                        states[other_index].connected_count += 1;

                        stack.push(other_index);
                        pos_stack.push(pos_dir(mgx, mgy, dir));

                        let mut small_grid = FixedGrid::new(30, 30, b'.');

                        states[index].tile.put_onto_grid(&mut small_grid, 10, 10);
                        let (ox, oy) = pos_dir(10, 10, dir);
                        states[other_index].tile.put_onto_grid(&mut small_grid, ox, oy);

                        println!("{:?}", states[index].tile.sides);
                        println!("{:?}", states[other_index].tile.sides);
                        println!("{} == {}", states[other_index].tile.sides[target_dir], side);
                        println!("{} == {}", states[other_index].tile.flipped_sides[target_dir], side);
                        for y in 0..small_grid.height() {
                            for x in 0..small_grid.width() {
                                print!("{}", small_grid.get(x, y) as u8 as char);
                            }
                            println!();
                        }
                        println!();

                        continue 'connect_loop;
                    }
                }
                */
            }
        }
    }

    (
        states.iter().enumerate()
            .filter(|(_, s)| s.connected_count == 2)
            .map(|(i, s)| {
                //println!("{:?}", s);
                tiles[i].id
            })
            .product(),
        states
    )
}

fn assemble_image(states: &[TileState]) -> FixedGrid<u8> {
    let top_left = states.iter().position(|s| s.connected == [true, true, false, false]).unwrap();
    let h_count = count_tiles(states, top_left, RIGHT);
    let v_count = count_tiles(states, top_left, DOWN);

    let mut grid = FixedGrid::new(h_count * 8, v_count * 8, b'#');

    let mut line_start = top_left;
    for i in 0..v_count {
        let y = i * 8;

        let mut current = line_start;
        for j in 0..h_count {
            let x = j * 8;

            states[current].tile.put_onto_grid(&mut grid, x, y);

            current = states[current].connections[RIGHT];
        }

        line_start = states[line_start].connections[DOWN];
    }

    grid
}

fn part2(grid: &FixedGrid<u8>) -> u64 {
    let mut grid = grid.clone();

    for nessie in NESSIES.iter() {
        let mut count = 0;

        for x in 0..grid.width() {
            for y in 0..grid.height() {
                let b = grid.get(x, y);
                if b != b'#' {
                    continue;
                }

                let mut found = true;
                for (x_offset, y_offset) in nessie.iter() {
                    let x = (x as isize) + x_offset;
                    let y = (y as isize) + y_offset;

                    if let Some(b) = grid.get_safe(x as usize, y as usize) {
                        if b != b'#' {
                            found = false;
                            break;
                        }
                    } else {
                        found = false;
                        break;
                    }
                }

                if found {
                    count += 1;

                    grid.set(x, y, b'O');
                    for (x_offset, y_offset) in nessie.iter() {
                        let x = (x as isize) + x_offset;
                        let y = (y as isize) + y_offset;

                        grid.set(x as usize, y as usize, b'O');
                    }
                }
            }
        }

        if count > 0 {
            break
        }
    }

    grid.data().iter().filter(|b| **b == b'#').count() as u64
}


fn count_tiles(states: &[TileState], from: usize, direction: usize) -> usize {
    let mut count = 1;
    let mut current = from;

    while states[current].connected[direction] {
        count += 1;

        current = states[current].connections[direction];
    }

    count
}

const NESSIES: [[(isize, isize); 14]; 8] = [
    [(1, 1), (4, 1), (5, 0), (6, 0), (7, 1), (10, 1), (11, 0), (12, 0), (13, 1), (16, 1), (17, 0), (18, -1), (18, 0), (19, 0)],
    [(-1, 1), (-4, 1), (-5, 0), (-6, 0), (-7, 1), (-10, 1), (-11, 0), (-12, 0), (-13, 1), (-16, 1), (-17, 0), (-18, -1), (-18, 0), (-19, 0)],
    [(-1, -1), (-4, -1), (-5, 0), (-6, 0), (-7, -1), (-10, -1), (-11, 0), (-12, 0), (-13, -1), (-16, -1), (-17, 0), (-18, 1), (-18, 0), (-19, 0)],
    [(1, -1), (4, -1), (5, 0), (6, 0), (7, -1), (10, -1), (11, 0), (12, 0), (13, -1), (16, -1), (17, 0), (18, 1), (18, 0), (19, 0)],
    [(-1, 1), (-1, 4), (0, 5), (0, 6), (-1, 7), (-1, 10), (0, 11), (0, 12), (-1, 13), (-1, 16), (0, 17), (1, 18), (0, 18), (0, 19)],
    [(1, 1), (1, 4), (0, 5), (0, 6), (1, 7), (1, 10), (0, 11), (0, 12), (1, 13), (1, 16), (0, 17), (-1, 18), (0, 18), (0, 19)],
    [(1, -1), (1, -4), (0, -5), (0, -6), (1, -7), (1, -10), (0, -11), (0, -12), (1, -13), (1, -16), (0, -17), (-1, -18), (0, -18), (0, -19)],
    [(-1, -1), (-1, -4), (0, -5), (0, -6), (-1, -7), (-1, -10), (0, -11), (0, -12), (-1, -13), (-1, -16), (0, -17), (1, -18), (0, -18), (0, -19)],
];

#[derive(Debug, Copy, Clone)]
struct TileState {
    connected: [bool; 4],
    connected_count: u32,
    connections: [usize; 4],
    tile: Tile,
}

impl TileState {
    fn from(tile: &Tile) -> TileState {
        TileState{
            connected: [false; 4],
            connections: [0; 4],
            connected_count: 0,
            tile: tile.clone(),
        }
    }
}

fn parse_input(input: &str) -> Vec<Tile> {
    let mut tiles = Vec::with_capacity(64);
    let mut current_tile = Tile::new();
    let mut parsing_id = true;
    let mut parsing_pos = 0usize;

    for line in input.lines() {
        if parsing_id {
            if line.len() == 0 {
                break;
            }

            current_tile.id = parse_u64(&line[5..line.len() - 1]);
            parsing_id = false;
        } else if line.len() == 0 {
            current_tile.generate_sides();
            tiles.push(current_tile);

            current_tile = Tile::new();

            parsing_id = true;
            parsing_pos = 0;
        } else {
            current_tile.pixels[parsing_pos..parsing_pos+line.len()].copy_from_slice(line.as_bytes());
            parsing_pos += line.len();
        }
    }

    tiles
}

fn rotate_right(dir: usize, amount: usize) -> usize {
    let r = dir + amount;
    if r > 3 {
        r - 4
    } else {
        r
    }
}

#[derive(Debug, Clone, Copy)]
struct Tile {
    id: u64,
    sides: [u64; 4],
    flipped_sides: [u64; 4],
    pixels: [u8; 100],
    flipped_h: bool,
    flipped_v: bool,
}

impl Tile {
    fn rotate(&mut self, n: usize) {
        if n == 0 {
            return;
        }

        let mut new_pixels = [0u8; 100];
        for i in 0..100 {
            let j = TILE_INDEX_ROTATED[n - 1][i];
            new_pixels[i] = self.pixels[j];
        }
        self.pixels = new_pixels;

        self.generate_sides();
    }

    fn flip_h(&mut self) {
        self.flipped_h = !self.flipped_h;

        let mut new_pixels = [0u8; 100];
        for i in 0..100 {
            let y = i / 10;
            let x = i % 10;
            let j = (y * 10) + (9 - x);

            new_pixels[i] = self.pixels[j]
        }
        self.pixels = new_pixels;

        self.generate_sides();
    }

    fn flip_v(&mut self) {
        self.flipped_v = !self.flipped_v;

        let mut new_pixels = [0u8; 100];
        for i in 0..100 {
            let y = i / 10;
            let x = i % 10;
            let j = ((9 - y) * 10) + x;

            new_pixels[i] = self.pixels[j]
        }
        self.pixels = new_pixels;

        self.generate_sides();
    }

    fn put_onto_grid(&self, grid: &mut FixedGrid<u8>, x: usize, y: usize) {
        for y2 in 0..8 {
            for x2 in 0..8 {
                let i = ((y2+1) * 10) + x2 + 1;

                grid.set(x+x2, y+y2, self.pixels[i]);
            }
        }
    }

    fn put_padded(&self, grid: &mut FixedGrid<u8>, x: usize, y: usize) {
        for i in 0..100 {
            let x2 = i % 10;
            let y2 = i / 10;

            grid.set(x + x2, y + y2, self.pixels[i]);
        }
    }

    fn generate_sides(&mut self) {
        self.sides = [0; 4];
        self.flipped_sides = [0; 4];

        for i in 0..10 {
            let bit = 1 << i;
            let flipped_bit = 1 << 9 - i;

            if self.pixels[9 - i] == b'#' {
                self.sides[UP] |= bit;
                self.flipped_sides[UP] |= flipped_bit;
            }
            if self.pixels[99 - i] == b'#' {
                self.sides[DOWN] |= bit;
                self.flipped_sides[DOWN] |= flipped_bit;
            }
            if self.pixels[i * 10] == b'#' {
                self.sides[LEFT] |= bit;
                self.flipped_sides[LEFT] |= flipped_bit;
            }
            if self.pixels[(i * 10) + 9] == b'#' {
                self.sides[RIGHT] |= bit;
                self.flipped_sides[RIGHT] |= flipped_bit;
            }
        }
    }

    fn new() -> Tile {
        Tile{id: 0, sides: [0; 4], flipped_sides: [0; 4], pixels: [0; 100], flipped_h: false, flipped_v: false}
    }
}