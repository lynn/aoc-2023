use std::collections::HashMap;

use crate::utils::grid::Grid;

pub fn light(
    grid: &Grid<u8>,
    energized: &mut HashMap<(i64, i64), u8>,
    x: i64,
    y: i64,
    dx: i64,
    dy: i64,
) {
    if !grid.in_range(x, y) {
        return;
    }

    let bit = match (dx, dy) {
        (1, 0) => 1,
        (-1, 0) => 2,
        (0, 1) => 4,
        (0, -1) => 8,
        _ => panic!(),
    };
    let old = energized.get(&(x, y)).unwrap_or(&0);
    if bit & old != 0 {
        return;
    }
    energized.insert((x, y), bit | old);

    match grid.get(x, y) {
        b'|' if dx != 0 => {
            light(grid, energized, x, y - 1, 0, -1);
            light(grid, energized, x, y + 1, 0, 1);
        }
        b'-' if dy != 0 => {
            light(grid, energized, x - 1, y, -1, 0);
            light(grid, energized, x + 1, y, 1, 0);
        }
        b'/' => light(grid, energized, x - dy, y - dx, -dy, -dx),
        b'\\' => light(grid, energized, x + dy, y + dx, dy, dx),
        _ => light(grid, energized, x + dx, y + dy, dx, dy),
    }
}

fn count_energized(grid: &Grid<u8>, x: i64, y: i64, dx: i64, dy: i64) -> usize {
    let mut energized: HashMap<(i64, i64), u8> = HashMap::new();
    light(&grid, &mut energized, x, y, dx, dy);
    energized.len()
}

fn most_energized(grid: &Grid<u8>) -> usize {
    let mut best = 0;
    let w = grid.width() as i64;
    let h = grid.height() as i64;
    for x in 0..w {
        best = best.max(count_energized(&grid, x, 0, 0, 1));
        best = best.max(count_energized(&grid, x, h - 1, 0, -1));
    }
    for y in 0..h {
        best = best.max(count_energized(&grid, 0, y, 1, 0));
        best = best.max(count_energized(&grid, w - 1, y, -1, 0));
    }
    best
}

pub fn main(input: &str) {
    let grid = Grid::<u8>::parse(input);
    println!("*  {}", count_energized(&grid, 0, 0, 1, 0));
    println!("** {}", most_energized(&grid));
}
