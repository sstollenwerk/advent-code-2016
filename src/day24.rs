use std::collections::HashSet;

use num_complex::Complex;
use pathfinding::directed::dijkstra::dijkstra;
use pathfinding::directed::dijkstra::dijkstra_all;

type Num = i32;

type Position = Complex<Num>;

type Floor = HashSet<Position>;
type Interest = Vec<Position>;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct State {
    to_see: Interest,
    current: Position,
}

impl State {
    fn new(mut to_see: Interest, current: Position) -> State {
        to_see.sort_by_key(|k| (k.re, k.im));
        State { to_see, current }
    }
}

fn parse(s: &str) -> (Floor, State) {
    let mut interest = Interest::new();
    let mut floor = Floor::new();
    let mut current = None;
    for (y, r) in s.trim().lines().enumerate() {
        for (x, c) in r.chars().enumerate() {
            let p = Position::new(x.try_into().unwrap(), y.try_into().unwrap());

            if c != '#' {
                floor.insert(p);
            }

            if c == '0' {
                current = Some(p);
            } else if c.is_ascii_digit() {
                interest.push(p);
            }
        }
    }

    let state = State::new(interest, current.unwrap());

    (floor, state)
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

fn base_succ(floor: &Floor, p: &Position) -> Vec<(Position, Num)> {
    let nexts: HashSet<Position> = adj(p).into_iter().collect();
    floor.intersection(&nexts).map(|x| (*x, 1)).collect()
}

fn successors(floor: &Floor, s: &State) -> Vec<(State, Num)> {
    let all_costs = dijkstra_all(&s.current, |&p| base_succ(floor, &p));

    let res: Vec<_> = s
        .to_see
        .iter()
        .filter(|x| x != &&s.current)
        .map(|p| {
            (
                State::new(s.to_see.iter().filter(|x| x != &p).copied().collect(), *p),
                all_costs[p].1,
            )
        })
        .collect();

    res
}

fn success(s: &State) -> bool {
    s.to_see.is_empty()
}

pub fn part1(s: &str) -> Num {
    let (floor, state) = parse(s);

    let start = state;
    let succ = |s: &State| successors(&floor, &s.clone());

    let res = dijkstra(&start, succ, success);

    res.unwrap().1
}
pub fn part2(s: &str) -> Num {
    let (floor, state) = parse(s);
    let mut to_see = state.to_see;
    to_see.push(state.current);
    let state = State::new(to_see, state.current);

    let succ = |s: &State| successors(&floor, &s.clone());

    let initial = state.current;

    let win = |s: &State| success(s) && s.current == initial;

    let res = dijkstra(&state, succ, win);

    res.unwrap().1
}
