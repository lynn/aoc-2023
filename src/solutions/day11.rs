use crate::utils::grid::Grid;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    Space,
    Galaxy,
}

impl From<u8> for Tile {
    fn from(value: u8) -> Self {
        match value {
            b'.' => Tile::Space,
            b'#' => Tile::Galaxy,
            _ => panic!("Bad tile: {value}"),
        }
    }
}

pub struct Universe {
    galaxies: Vec<(i64, i64)>,
    expansion_factor: usize,
    expand_column: Vec<bool>,
    expand_row: Vec<bool>,
}

impl Universe {
    fn parse(input: &str) -> Universe {
        let grid = Grid::<Tile>::parse(input);
        let galaxies = grid.byte_positions(b'#').collect();
        let w = grid.width() as i64;
        let h = grid.height() as i64;
        let expand_column = (0..w)
            .map(|x| (0..h).all(|y| grid.get(x, y) == Tile::Space))
            .collect();
        let expand_row = (0..h)
            .map(|y| (0..w).all(|x| grid.get(x, y) == Tile::Space))
            .collect();
        Universe {
            galaxies,
            expansion_factor: 2,
            expand_column,
            expand_row,
        }
    }

    fn size(&self, x: i64, y: i64) -> usize {
        if self.expand_column[x as usize] || self.expand_row[y as usize] {
            self.expansion_factor
        } else {
            1
        }
    }

    fn distance(&self, p1: (i64, i64), p2: (i64, i64)) -> usize {
        let mut dist = 0;
        let (mut p1x, mut p1y) = p1;
        let (p2x, p2y) = p2;

        while p1x != p2x {
            p1x += (p2x - p1x).signum();
            dist += self.size(p1x, p1y);
        }

        while p1y != p2y {
            p1y += (p2y - p1y).signum();
            dist += self.size(p1x, p1y);
        }
        dist
    }

    fn shortest_path_sum(&self) -> usize {
        let mut sum = 0;
        for (i, g1) in self.galaxies.iter().enumerate() {
            for g2 in &self.galaxies[i + 1..] {
                sum += self.distance(*g1, *g2);
            }
        }
        sum
    }
}

pub fn main(input: &str) {
    let mut universe = Universe::parse(input);

    println!("*  {}", universe.shortest_path_sum());
    universe.expansion_factor = 1_000_000;
    println!("** {}", universe.shortest_path_sum());
}
