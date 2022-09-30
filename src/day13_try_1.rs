use num_complex::Complex;

use pathfinding::directed::astar::astar;

use pathfinding::directed::dijkstra::dijkstra;

type Num = i32;

type Position = Complex<Num>;

type Edge = (Position, u32);

const DESTINATION: Position = Position::new(31, 39);

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
    adj(p).into_iter().filter(|c| valid(c) ).map(|c| (c, 1)).collect()
}

fn heuristic(p: &Position, dest: &Position) -> u32 {
    (dest - p).l1_norm() as u32
}

fn parse(s: &str) -> Num {
    s.parse().unwrap()
}

pub fn part1(s: &str) -> u32 {
    let r = parse(s);
    println!("{:?}", &r);
    let start = Position::new(1, 1);
    let dest = Position::new(7, 4);
    //  let dest = DESTINATION;
    let valid = |&p| is_valid(&r, &p, &dest);
    let neighbours = |&p| neighbours1(&p, &valid);
    let heur = |&p| heuristic(&p, &dest);
    let success = |&p| heur(&p) == 0;
    let (path, cost) = astar(&start, neighbours, heur, success).unwrap();
    println!("{:?}", &path);
    cost
}

pub fn part2(s: &str) -> u32 {
    todo!();
}
