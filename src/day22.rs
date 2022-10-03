use std::collections::HashMap;

use itertools::iproduct;
use num_complex::Complex;
use pathfinding::directed::astar::astar;

type Position = Complex<i32>;

const GOAL: Position = Position::new(0, 0);

#[derive(Debug, Clone, Eq, PartialEq, Hash, Copy)]
struct Node {
    place: Position,
    size: u32,
    used: u32,
    avail: u32,
    use_per: u32,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct State {
    vals: Vec<Node>,
    desired: Position,
}

// vals is vec because need to implement hash

impl State {
    fn new(vals: Vec<Node>, desired: Position) -> State {
        State { vals, desired }
    }
}

impl Node {
    fn new(place: Position, size: u32, used: u32, avail: u32, use_per: u32) -> Node {
        Node {
            place,
            size,
            used,
            avail,
            use_per,
        }
    }

    fn from_raw(base: &str) -> Node {
        let parts: Vec<u32> = base
            .replace(['T', '%', '%', 'x', 'y'], "")
            .replace("/dev/grid/node-", "")
            .replace('-', " ")
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        let p = Position::new(parts[1] as i32, parts[0] as i32);

        Node::new(p, parts[2], parts[3], parts[4], parts[5])
    }
}

fn parse(base: &str) -> Vec<Node> {
    base.lines().skip(2).map(Node::from_raw).collect()
}

fn fits(a: &Node, b: &Node) -> bool {
    a.used > 0 && a != b && a.used <= b.avail
}

fn all_fit(xs: &[Node]) -> usize {
    iproduct!(xs, xs).filter(|(a, b)| fits(a, b)).count()
}

pub fn part1(s: &str) -> usize {
    let r = parse(s);

    all_fit(&r)
}

fn adj(p: &Position) -> Vec<Position> {
    [
        Position::new(0, 1),
        Position::new(1, 0),
        Position::new(0, -1),
        Position::new(-1, 0),
    ]
    .iter()
    .map(|c| c + p)
    .collect()
}

fn successors(s: &State) -> Vec<State> {
    let blanks: Vec<Position> = s
        .vals
        .iter()
        .filter(|n| n.used == 0)
        .map(|x| (x.place))
        .collect();
    assert!(blanks.len() == 1);
    let blank: Position = blanks[0];

    let positions: HashMap<Position, Node> = s.vals.iter().map(|x| (x.place, *x)).collect();

    let cur = positions[&blank];

    let binding = adj(&blank);

    let posses: Vec<_> = binding
        .iter()
        .filter_map(|c| positions.get(c))
        .filter(|p| p.used <= cur.avail)
        .collect();

    let mut states: Vec<State> = Vec::new();
    for node_ in posses.iter() {
        let node: Node = *&(*node_).clone();

        let mut pos = positions.clone();

        let p = node.place;

        let amt = node.used;

        let mut tmp = pos[&blank];
        tmp.used = amt;
        tmp.avail = tmp.avail - amt;

        pos.insert(blank, tmp);

        let mut tmp = pos[&p];
        tmp.used = 0;
        tmp.avail = tmp.size;

        pos.insert(p, tmp);

        let mut places: Vec<Node> = pos.values().copied().collect();

        places.sort_by_key(|n| (n.place.re, n.place.im));

        let desired = if p == s.desired { blank } else { s.desired };

        states.push(State::new(places, desired));
    }

    states
}

fn successors_weighted(s: &State) -> Vec<(State, i32)> {
    successors(s).into_iter().map(|x| (x, 1)).collect()
}

fn heuristic(s: &State) -> i32 {
    let blanks: Vec<Position> = s
        .vals
        .iter()
        .filter(|n| n.used == 0)
        .map(|x| (x.place))
        .collect();
    assert!(blanks.len() == 1);
    let blank: Position = blanks[0];

    (blank - s.desired).l1_norm() + (GOAL - s.desired).l1_norm()
}

pub fn part2(s: &str) -> i32 {
    let mut r = parse(s);

    r.sort_by_key(|n| (n.place.re, n.place.im));

    let capacity =  r.iter().map(|c| c.size).filter(|c| c < &200).min().unwrap();
    let used =  r.iter().map(|c| c.used).filter(|c| c < &200).max().unwrap();

    assert!(capacity >=  used);
    // let positions:HashMap<Position, Node> =  r.into_iter().map(|x| (x.place, x)  ).collect()  ;
    let desired_ = r
        .iter()
        .map(|n| n.place)
        .filter(|&c| c.re == 0)
        .map(|c| c.im)
        .max();
    let desired = Position::new(0, desired_.unwrap());

    let state = State::new(r, desired);


    // let result = bfs(&state, successors, |p| p.desired == GOAL).unwrap();
    let result = astar(&state, successors_weighted, heuristic, |p| {
        p.desired == GOAL
    })
    .unwrap();

    result.1
}
