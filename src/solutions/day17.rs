use crate::utils::grid::Grid;
use pathfinding::prelude::astar;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct State {
    x: i64,
    y: i64,
    dx: i64,
    dy: i64,
    steps: usize,
}

fn successors(
    grid: &Grid<u8>,
    previous: State,
    min_steps: usize,
    max_steps: usize,
) -> impl Iterator<Item = (State, usize)> + '_ {
    vec![(-1, 0), (1, 0), (0, -1), (0, 1)]
        .into_iter()
        .filter_map(move |(dx, dy)| {
            let x = previous.x + dx;
            let y = previous.y + dy;
            let turn = (dx, dy) != (previous.dx, previous.dy);
            if turn && previous.steps < min_steps {
                return None;
            }
            let steps = if turn { 1 } else { previous.steps + 1 };
            if steps > max_steps || !grid.in_range(x, y) {
                return None;
            }
            let state = State {
                x,
                y,
                dx,
                dy,
                steps,
            };
            let weight = grid.get(x, y) - b'0';
            Some((state, weight as usize))
        })
}

fn heat_loss(grid: &Grid<u8>, min_steps: usize, max_steps: usize) -> usize {
    let w = grid.width() as i64;
    let h = grid.height() as i64;
    let start = State {
        x: 0,
        y: 0,
        dx: 0,
        dy: 0,
        steps: min_steps,
    };
    astar(
        &start,
        |s| successors(&grid, s.clone(), min_steps, max_steps),
        |s| ((w - 1).abs_diff(s.x) + (h - 1).abs_diff(s.y)) as usize,
        |s| (s.x, s.y) == (w - 1, h - 1),
    )
    .unwrap()
    .1
}

pub fn main(input: &str) {
    let grid = Grid::<u8>::parse(input);
    println!("*  {:?}", heat_loss(&grid, 1, 3));
    println!("** {:?}", heat_loss(&grid, 4, 10));
}
