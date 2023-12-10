use std::collections::HashSet;

use crate::utils::grid::Grid;

pub struct Tile {
    directions: &'static [(i64, i64)],
}

const NORTH: (i64, i64) = (0, -1);
const SOUTH: (i64, i64) = (0, 1);
const WEST: (i64, i64) = (-1, 0);
const EAST: (i64, i64) = (1, 0);

impl From<u8> for Tile {
    fn from(value: u8) -> Self {
        let directions: &'static [(i64, i64)] = match value {
            b'S' => &[NORTH, WEST, EAST, SOUTH],
            b'|' => &[NORTH, SOUTH],
            b'-' => &[WEST, EAST],
            b'L' => &[NORTH, EAST],
            b'J' => &[NORTH, WEST],
            b'7' => &[SOUTH, WEST],
            b'F' => &[SOUTH, EAST],
            b'.' => &[],

            _ => panic!("Bad pipe char: {value}"),
        };
        Tile { directions }
    }
}

pub struct Maze {
    grid: Grid<Tile>,
}

impl Maze {
    fn parse(input: &str) -> Maze {
        Maze {
            grid: Grid::parse(input),
        }
    }

    fn main_loop_and_distance(&self) -> (HashSet<(i64, i64)>, usize) {
        let start = self.grid.find_byte(b'S').unwrap();
        let mut visited: HashSet<(i64, i64)> = HashSet::new();
        let mut frontier = vec![start];
        let mut distance = 0;

        while !frontier.is_empty() {
            let mut new_frontier = vec![];
            for (x, y) in frontier {
                visited.insert((x, y));
                let tile = self.grid.get(x, y);
                for (dx, dy) in tile.directions {
                    if self.grid.in_range(x + dx, y + dy) {
                        let other_tile = self.grid.get(x + dx, y + dy);
                        if other_tile.directions.contains(&(-dx, -dy))
                            && !visited.contains(&(x + dx, y + dy))
                        {
                            new_frontier.push((x + dx, y + dy));
                        }
                    }
                }
            }
            frontier = new_frontier;
            distance += 1;
        }
        (visited, distance - 1)
    }

    fn enclosed_area(&self, path: HashSet<(i64, i64)>) -> usize {
        let mut area = 0;

        // Basically https://en.wikipedia.org/wiki/Point_in_polygon#Ray_casting_algorithm
        for y in 0..self.grid.height() as i64 {
            let mut inside = false;
            for x in 0..self.grid.width() as i64 {
                if path.contains(&(x, y)) {
                    let tile = self.grid.get(x, y);
                    if tile.directions.contains(&NORTH) {
                        inside = !inside;
                    }
                } else if inside {
                    area += 1
                }
            }
        }
        area
    }
}

pub fn main(input: &str) {
    let maze = Maze::parse(input);
    let (main_loop, diameter) = maze.main_loop_and_distance();
    println!("*  {}", diameter);
    println!("** {}", maze.enclosed_area(main_loop))
}
