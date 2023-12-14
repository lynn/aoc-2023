use std::collections::HashMap;

use crate::utils::grid::Grid;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    Empty,
    Wall,
    Rock,
}

impl From<u8> for Tile {
    fn from(value: u8) -> Self {
        match value {
            b'.' => Tile::Empty,
            b'#' => Tile::Wall,
            b'O' => Tile::Rock,
            _ => panic!("Bad tile: {value}"),
        }
    }
}

fn roll_vertical(grid: &mut Grid<Tile>, rev: bool) -> usize {
    let mut load = 0;
    for x in 0..grid.width() as i64 {
        let mut was_wall = true;
        let mut new_y = 0;
        for mut y in 0..grid.height() as i64 {
            if rev {
                y = grid.height() as i64 - y - 1;
            }
            let tile = grid.get(x, y);
            if tile == Tile::Wall {
                was_wall = true;
            } else if was_wall {
                new_y = y;
                was_wall = false;
            }
            if tile == Tile::Rock {
                grid.set(x, y, b'.');
                grid.set(x, new_y, b'O');
                load += grid.height() - new_y as usize;
                new_y += if rev { -1 } else { 1 };
            }
        }
    }
    load
}

fn roll_north(grid: &mut Grid<Tile>) -> usize {
    roll_vertical(grid, false)
}

fn roll_south(grid: &mut Grid<Tile>) -> usize {
    roll_vertical(grid, true)
}

fn roll_horizontal(grid: &mut Grid<Tile>, rev: bool) -> usize {
    let mut load = 0;
    for y in 0..grid.height() as i64 {
        let mut was_wall = true;
        let mut new_x = 0;
        for mut x in 0..grid.width() as i64 {
            if rev {
                x = grid.width() as i64 - x - 1;
            }
            let tile = grid.get(x, y);
            if tile == Tile::Wall {
                was_wall = true;
            } else if was_wall {
                new_x = x;
                was_wall = false;
            }
            if tile == Tile::Rock {
                grid.set(x, y, b'.');
                grid.set(new_x, y, b'O');
                load += grid.height() - y as usize;
                new_x += if rev { -1 } else { 1 };
            }
        }
    }
    load
}

fn roll_west(grid: &mut Grid<Tile>) -> usize {
    roll_horizontal(grid, false)
}

fn roll_east(grid: &mut Grid<Tile>) -> usize {
    roll_horizontal(grid, true)
}

fn spin_cycle(grid: &mut Grid<Tile>) -> usize {
    roll_north(grid);
    roll_west(grid);
    roll_south(grid);
    roll_east(grid)
}

pub fn main(input: &str) {
    let mut grid = Grid::<Tile>::parse(input);
    let load = roll_north(&mut grid);
    println!("*  {}", load);

    // Finish the first spin cycle
    roll_west(&mut grid);
    roll_south(&mut grid);
    roll_east(&mut grid);

    let mut seen = HashMap::<Grid<Tile>, usize>::new();
    let mut loads = HashMap::<usize, usize>::new();
    for i in 2.. {
        let load = spin_cycle(&mut grid);
        loads.insert(i, load);
        if let Some(past) = seen.insert(grid.clone(), i) {
            let period = i - past;
            // Find the load for the time t within the period that's equal to
            // 1_000_000_000 modulo the period.
            let t = past + (1_000_000_000 - past) % period;
            println!("** {}", loads.get(&t).unwrap());
            break;
        }
    }
}
