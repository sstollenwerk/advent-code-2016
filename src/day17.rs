use core::hash::Hash;
use std::collections::VecDeque;

use md5::{Digest, Md5};
use num_complex::Complex;
use pathfinding::directed::bfs::bfs;

use crate::helper::as_u4;

type Position = Complex<i32>;

type Moves = Vec<char>;

use std::collections::HashMap;

const DOWN: Position = Position::new(0, 1);
const RIGHT: Position = Position::new(1, 0);
const LEFT: Position = Position::new(-1, 0);
const UP: Position = Position::new(0, -1);

//const DIRECTIONS: HashMap<char, Position> = HashMap::from([('U', UP), ('D', DOWN), ('L', LEFT), ('R', RIGHT)]);
// cannot call non-const fn `<HashMap<char, Complex<i32>> as From<[(char, Complex<i32>); 4]>>::from` in constants

fn valid_directions(s: &str) -> Moves {
    let dirs = vec!['U', 'D', 'L', 'R'];
    find_hash(s)
        .into_iter()
        .zip(dirs.into_iter())
        .filter(|(n, _)| n >= &11 && n <= &15)
        .map(|t| t.1)
        .collect()
}

fn to_position(moves: &Moves) -> Position {
    let directions = HashMap::from([('U', UP), ('D', DOWN), ('L', LEFT), ('R', RIGHT)]);
    let deltas = moves.iter().map(|c| directions.get(c).unwrap());
    deltas.sum()
}

fn valid_position(p: &Position) -> bool {
    p.re >= 0 && p.im >= 0 && p.re <= 3 && p.im <= 3
    // Turns out this was fine for part2.
}

fn success(moves: &Moves, dest: &Position) -> bool {
    &to_position(moves) == dest
}

fn successors(moves: &Moves, passcode: &str) -> Vec<Moves> {
    let m = passcode.to_owned() + &(moves.iter().map(|c| *c).collect::<String>());
    let mut res = Vec::new();
    for d in valid_directions(&m) {
        let mut p = moves.clone();
        p.push(d);
        if valid_position(&to_position(&p)) {
            res.push(p);
        }
    }
    res
}

fn find_hash(s: &str) -> Vec<u8> {
    let mut hasher = Md5::new();
    hasher.update(s);

    as_u4(&hasher.finalize())
}

fn longest_via_bfs<N, FN, IN, FS>(start: &N, mut successors: FN, mut success: FS) -> Option<N>
where
    N: Eq + Hash + Clone,
    FN: FnMut(&N) -> IN,
    IN: IntoIterator<Item = N>,
    FS: FnMut(&N) -> bool,
{
    // signature based on  pathfinding::directed::bfs::bfs
    let mut res: Option<N> = None;
    let mut posses: VecDeque<N> = VecDeque::new();
    posses.push_front(start.clone());

    while let Some(node) = posses.pop_back() {
        let s = success(&node);
        if s {
            res = Some(node.clone());
        } else {
            let nexts = successors(&node);
            for n in nexts {
                posses.push_front(n);
            }
        }
    }
    res
}

pub fn part1(s: &str) -> String {
    let nexts = |m: &Moves| successors(&m.clone(), s);
    let won = |m: &Moves| success(&m.clone(), &Position::new(3, 3));
    let r = bfs(&Vec::new(), nexts, won);
    r.unwrap().last().unwrap().iter().collect()
}

pub fn part2(s: &str) -> usize {
    let nexts = |m: &Moves| successors(&m.clone(), s);
    let won = |m: &Moves| success(&m.clone(), &Position::new(3, 3));
    let r = longest_via_bfs(&Vec::new(), nexts, won);
    r.unwrap().len()
}
