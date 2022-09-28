use std::collections::HashSet;

use num_complex::Complex;

type Num = i32;

type Position = Complex<Num>;

type Delta = (Position, Num);

type PlaceInfo = (Position, Position);
// easy in short term to treat position and rotation the same
// lets see if that holds true

const NORTH: Position = Position::new(0, 1);
const LEFT: Position = Position::new(0, 1);
const RIGHT: Position = Position::new(0, -1);

const ORIGIN: Position = Position::new(0, 0);

fn read_part(part: &str) -> Delta {
    let d = match part.chars().nth(0).unwrap() {
        'L' => LEFT,
        'R' => RIGHT,
        _ => unreachable!(),
    };

    let dist: Num = part.trim()[1..].parse().unwrap();
    (d, dist)
}

fn parse(base: &String) -> Vec<Delta> {
    base.split(", ").map(read_part).collect()
}

fn step(pos: PlaceInfo, move_: &Delta) -> PlaceInfo {
    let (d, p) = pos;
    let (rot, delta) = move_;
    let new_r = (d * rot);
    (new_r, p + new_r.scale(*delta))
}

pub fn part1(base: &String) -> i32 {
    let data = parse(base);

    let start = (NORTH, ORIGIN);
    let final_position = data.iter().fold(start, step);

    final_position.1.l1_norm()
}

fn first_dupe(start: &PlaceInfo, moves: &Vec<Delta>) -> Position {
    let mut seen = HashSet::new();

    let (mut d, mut p) = start.clone();
    for (rot, delta) in moves.iter() {
        d *= rot;
        for _ in 0..*delta {
            p += d;
            if !(seen.insert(p)) {
                return p;
            }
        }
    }

    unreachable!()
}

pub fn part2(base: &String) -> i32 {
    let data = parse(base);

    let start = (NORTH, ORIGIN);

    first_dupe(&start, &data).l1_norm()
}
