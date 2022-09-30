use num_complex::Complex;

use pathfinding::directed::astar::astar;

use pathfinding::directed::dijkstra::dijkstra_partial;
type Num = i32;

type Position = Complex<Num>;

type Edge = (Position, u32);

const DESTINATION: Position = Position::new(31, 39);
const START: Position = Position::new(1, 1);
const FAVOURITE: Num = 1352;
const TOCHECK: Num = 50;

fn is_even(n: u32) -> bool {
    (n % 2) == 0
}

fn is_valid(p: &Position) -> bool {
    let fav = FAVOURITE;
    let dest = DESTINATION;
    let (x, y) = (p.re, p.im);
    let m = (x * x + 3 * x + 2 * x * y + y + y * y) + fav;
    (p == &dest) || ((x >= 0) && (y >= 0) && is_even(m.count_ones()))
}

fn adj(p: &Position) -> Vec<Position> {
    let vals = vec![
        Position::new(1, 0),
        Position::new(0, 1),
        Position::new(-1, 0),
        Position::new(0, -1),
    ];
    vals.iter().map(|c| c + p).collect()
}

fn neighbours(p: &Position) -> Vec<Edge> {
    adj(p)
        .into_iter()
        .filter(is_valid)
        .map(|c| (c, 1))
        .collect()
}

fn heuristic(p: &Position) -> u32 {
    let dest = DESTINATION;
    (dest - p).l1_norm() as u32
}
fn success(p: &Position) -> bool {
    heuristic(p) == 0
}

fn stop(p: &Position) -> bool {
    let start = START;
    (start - p).l1_norm() > TOCHECK
}

pub fn part1(_s: &str) -> u32 {
    let start = Position::new(1, 1);
    let heur = heuristic;
    let (_, cost) = astar(&start, neighbours, heur, success).unwrap();
    cost
}

pub fn part2(_s: &str) -> usize {
    let start = Position::new(1, 1);

    let (paths, _) = dijkstra_partial(&start, neighbours, stop);
    let costs = paths.values().map(|p| p.1);
    costs.filter(|c| c <= &(TOCHECK as u32)).count() + 1
}
