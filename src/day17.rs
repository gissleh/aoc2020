use common::aoc::{print_result, print_time, run_many, run_once, load_input_bytes};

const NEWLINE: u8 = '\n' as u8;
const PIXEL_OFF: u8 = '.' as u8;
const PIXEL_ON: u8 = '#' as u8;

fn main() {
    let (input, dur_load) = run_once(|| load_input_bytes("day17"));

    print_time("Load", dur_load);

    let (grid, dur_parse) = run_many(1000, || CubeGrid::parse_2d(&input));
    let (res_part1, dur_part1) = run_many(1000, || part1(&grid));
    let (res_part2, dur_part2) = run_many(100, || part2(&grid));

    print_result("P1", res_part1);
    print_result("P2", res_part2);

    print_time("Parse", dur_parse);
    print_time("P1", dur_part1);
    print_time("P2", dur_part2);
    print_time("Total", dur_parse + dur_part1 + dur_part2);
}

fn part1(grid: &CubeGrid) -> usize {
    let grid = grid.padded3(7, PIXEL_OFF);
    let offsets = grid.neighbor_offsets3();
    let start = (grid.width * grid.height) + grid.width + 1;
    let end = grid.data.len() - (grid.width * grid.height) - (grid.width + 1);

    parts_common(grid, &offsets, start, end)
}

fn part2(grid: &CubeGrid) -> usize {
    let grid = grid.padded4(7, PIXEL_OFF);
    let offsets = grid.neighbor_offsets4();
    let start = (grid.width * grid.height * grid.depth) + (grid.width * grid.height) + grid.width + 1;
    let end = grid.data.len() - (grid.width * grid.height * grid.depth) - (grid.width * grid.height) - (grid.width + 1);

    parts_common(grid, &offsets, start, end)
}

fn parts_common(mut grid: CubeGrid, offsets: &[usize], start: usize, end: usize) -> usize {
    let mut changes = Vec::with_capacity(grid.data.len());

    for _ in 0..6 {
        for i in start..end {
            let mut count = 0;
            let active = grid.data[i] == PIXEL_ON;
            for offset in offsets.iter() {
                if grid.data[i + offset] == PIXEL_ON {
                    count += 1;
                    if count == 4 {
                        break;
                    }
                }
            }
            if active && (count < 2 || count > 3) {
                changes.push((i, PIXEL_OFF));
            }
            if !active && count == 3 {
                changes.push((i, PIXEL_ON));
            }
        }

        for (i, v) in changes.iter() {
            grid.data[*i] = *v;
        }

        changes.clear();
    }

    grid.data.iter().filter(|n| **n == PIXEL_ON).count()
}

struct CubeGrid {
    data: Vec<u8>,
    width: usize,
    height: usize,
    depth: usize,
    hyper_depth: usize,
}

impl CubeGrid {
    fn neighbor_offsets3(&self) -> [usize; 26] {
        let neg1 = usize::max_value();
        let w = self.width;
        let nw = neg1 * w;
        let h = self.width;
        let wh = w * h;
        let nwh = w * h * neg1;

        [
            nwh-w-1, nwh-w, nwh-w+1,
            nwh - 1, nwh, nwh + 1,
            nwh+w-1, nwh+w, nwh+w+1,

            nw-1, nw, nw+1,
            neg1, /*0,*/ 1,
            w-1, w, w+1,

            wh-w-1, wh-w, wh-w+1,
            wh - 1, wh, wh + 1,
            wh+w-1, wh+w, wh+w+1,
        ]
    }

    fn neighbor_offsets4(&self) -> [usize; 80] {
        let neg1 = usize::max_value();
        let w = self.width;
        let nw = neg1 * w;
        let h = self.width;
        let d = self.depth;
        let wh = w * h;
        let nwh = wh * neg1;
        let whd = wh * d;
        let nwhd = whd * neg1;

        [
            nwhd+nwh-w-1, nwhd+nwh-w, nwhd+nwh-w+1,
            nwhd+nwh - 1, nwhd+nwh, nwhd+nwh + 1,
            nwhd+nwh+w-1, nwhd+nwh+w, nwhd+nwh+w+1,

            nwhd+nw-1, nwhd+nw, nwhd+nw+1,
            nwhd-1, nwhd, nwhd+1,
            nwhd+w-1, nwhd+w, nwhd+w+1,

            nwhd+wh-w-1, nwhd+wh-w, nwhd+wh-w+1,
            nwhd+wh - 1, nwhd+wh, nwhd+wh + 1,
            nwhd+wh+w-1, nwhd+wh+w, nwhd+wh+w+1,


            nwh-w-1, nwh-w, nwh-w+1,
            nwh - 1, nwh, nwh + 1,
            nwh+w-1, nwh+w, nwh+w+1,

            nw-1, nw, nw+1,
            neg1, /*0,*/ 1,
            w-1, w, w+1,

            wh-w-1, wh-w, wh-w+1,
            wh - 1, wh, wh + 1,
            wh+w-1, wh+w, wh+w+1,


            whd+nwh-w-1, whd+nwh-w, whd+nwh-w+1,
            whd+nwh - 1, whd+nwh, whd+nwh + 1,
            whd+nwh+w-1, whd+nwh+w, whd+nwh+w+1,

            whd+nw-1, whd+nw, whd+nw+1,
            whd-1, whd, whd+1,
            whd+w-1, whd+w, whd+w+1,

            whd+wh-w-1, whd+wh-w, whd+wh-w+1,
            whd+wh - 1, whd+wh, whd+wh + 1,
            whd+wh+w-1, whd+wh+w, whd+wh+w+1,
        ]
    }

    fn padded3(&self, padding: usize, value: u8) -> CubeGrid {
        let width = self.width + padding * 2;
        let height = self.height + padding * 2;
        let depth = self.depth + padding * 2;
        let mut data = vec![value; width * height * depth];

        for sz in 0..self.depth {
           for sy in 0..self.height {
               let source_start = (sz * self.width * self.height) + (sy * self.width);
               let source_end = source_start + self.width;
               let src = &self.data[source_start..source_end];

               let dest_start = ((sz+padding) * (width * height)) + ((sy+padding) * (width)) + padding;
               let dest_end = dest_start + self.width;
               let dst = &mut data[dest_start..dest_end];

               dst.copy_from_slice(src);
           }
        }

        CubeGrid{
            data, width, height, depth,
            hyper_depth: 1,
        }
    }

    fn padded4(&self, padding: usize, value: u8) -> CubeGrid {
        let width = self.width + padding * 2;
        let height = self.height + padding * 2;
        let depth = self.depth + padding * 2;
        let hyper_depth = self.hyper_depth + padding * 2;
        let mut data = vec![value; width * height * depth * hyper_depth];

        for sw in 0..self.hyper_depth {
            for sz in 0..self.depth {
                for sy in 0..self.height {
                    let source_start = (sw * self.width * self.height * self.depth) + (sz * self.width * self.height) + (sy * self.width);
                    let source_end = source_start + self.width;
                    let src = &self.data[source_start..source_end];

                    let dest_start = ((sw+padding) * (width * height * depth)) + ((sz+padding) * (width * height)) + ((sy+padding) * (width)) + padding;
                    let dest_end = dest_start + self.width;
                    let dst = &mut data[dest_start..dest_end];

                    dst.copy_from_slice(src);
                }
            }
        }

        CubeGrid{
            data, width, height, depth, hyper_depth
        }
    }

    fn parse_2d(input: &[u8]) -> CubeGrid {
        let width = input.iter().position(|b| *b == NEWLINE).unwrap();
        let height = input.len() / (width + 1);

        let mut data = vec![0u8; width * height];

        for i in 0..height {
            let src_pos = i * (width + 1);
            let src = &input[src_pos..src_pos+width];
            let dst_pos = i * width;
            let dst = &mut data[dst_pos..dst_pos+width];

            dst.copy_from_slice(src);
        }

        CubeGrid{
            data, width, height,
            depth: 1,
            hyper_depth: 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const START: &[u8] = ".#.
..#
###
".as_bytes();

    #[test]
    fn test_part1() {
        let grid = CubeGrid::parse_2d(START);

        assert_eq!(part1(&grid), 112);
    }

    #[test]
    fn test_part2() {
        let grid = CubeGrid::parse_2d(START);

        assert_eq!(part2(&grid), 848);
    }
}