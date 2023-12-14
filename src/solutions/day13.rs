use crate::utils::grid::Grid;

/// Is there a symmetry axis after the leftmost n columns?
fn column_symmetric(grid: &Grid<u8>, n: usize) -> bool {
    (0..grid.height()).all(|y| {
        (0..n).all(|x| {
            let k = 2 * n - 1 - x;
            k >= grid.width() || grid.get(x as i64, y as i64) == grid.get(k as i64, y as i64)
        })
    })
}

/// Is there a symmetry axis below the top n rows?
fn row_symmetric(grid: &Grid<u8>, n: usize) -> bool {
    (0..n).all(|y| {
        let k = 2 * n - 1 - y;
        k >= grid.height() || grid.row(y) == grid.row(k)
    })
}

fn symmetry_score(grid: &Grid<u8>, ignore: Option<usize>) -> Option<usize> {
    for x in 1..grid.width() {
        if column_symmetric(grid, x) && ignore != Some(x) {
            return Some(x);
        }
    }

    for y in 1..grid.height() {
        if row_symmetric(grid, y) && ignore != Some(100 * y) {
            return Some(100 * y);
        }
    }

    None
}

fn alternate_score(grid: &mut Grid<u8>) -> usize {
    let original_score = symmetry_score(grid, None);
    for x in 0..grid.width() as i64 {
        for y in 0..grid.height() as i64 {
            let original = grid.get(x, y);
            let smudge = original ^ (b'.' ^ b'#');
            grid.set(x, y, smudge);
            if let Some(score) = symmetry_score(grid, original_score) {
                return score;
            }
            grid.set(x, y, original);
        }
    }
    panic!("No smudge found");
}

pub fn main(input: &str) {
    let mut score = 0;
    let mut alt_score = 0;
    for paragraph in input.split("\n\n") {
        let mut grid = Grid::<u8>::parse(paragraph);
        score += symmetry_score(&grid, None).unwrap();
        alt_score += alternate_score(&mut grid);
    }
    println!("*  {score}");
    println!("** {alt_score}");
}
