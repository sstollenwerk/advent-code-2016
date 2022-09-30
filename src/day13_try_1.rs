use num_complex::Complex;

use pathfinding::directed::astar::astar;

use pathfinding::directed::dijkstra::dijkstra_partial;

type Num = i32;

type Position = Complex<Num>;

type Edge = (Position, u32);

const DESTINATION: Position = Position::new(31, 39);

const TOCHECK: Num = 50;

fn is_even(n: u32) -> bool {
    (n % 2) == 0
}

fn is_valid(off: &Num, p: &Position, dest: &Position) -> bool {
    let (x, y) = (p.re, p.im);
    let m = (x * x + 3 * x + 2 * x * y + y + y * y) + off;
    (p == dest) || ((x >= 0) && (y >= 0) && is_even(m.count_ones()))
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

fn neighbours1(p: &Position, valid: &dyn Fn(&Position) -> bool) -> Vec<Edge> {
    adj(p)
        .into_iter()
        .filter(|c| valid(c))
        .map(|c| (c, 1))
        .collect()
}

fn heuristic(p: &Position, dest: &Position) -> u32 {
    (dest - p).l1_norm() as u32
}

fn stop(p: &Position, start: &Position, to_check: Num) -> bool {
    (start - p).l1_norm() > to_check
}

fn parse(s: &str) -> Num {
    s.parse().unwrap()
}

pub fn part1(s: &str) -> u32 {
    let r = parse(s);
    let start = Position::new(1, 1);
    let dest = DESTINATION;
    let valid = |&p: &Position| is_valid(&r, &(p.clone()), &dest);
    let neighbours = |&p: &Position| neighbours1(&(p.clone()), &valid);
    let heur = |&p: &Position| heuristic(&(p.clone()), &dest);
    let success = |&p: &Position| heur(&(p.clone())) == 0;
    let (_, cost) = astar(&start, neighbours, heur, success).unwrap();
    cost
}

pub fn part2(s: &str) -> usize {
    let r = parse(s);
    let start = Position::new(1, 1);
    let dest = DESTINATION;
    let valid = |&p: &Position| is_valid(&r, &(p.clone()), &dest);
    let neighbours = |&p: &Position| neighbours1(&(p.clone()), &valid);
    let quit = |&p: &Position| stop(&(p.clone()), &start, TOCHECK);

    let (paths, _) = dijkstra_partial(&start, neighbours, quit);
    let costs = paths.values().map(|p| p.1);
    costs.filter(|c| c <= &(TOCHECK as u32)).count() + 1
}
