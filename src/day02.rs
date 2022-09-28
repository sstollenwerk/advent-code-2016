use std::collections::HashMap;

use cached::proc_macro::cached;
use itertools::iproduct;
use num_complex::Complex;

type Num = i32;

type Position = Complex<Num>;

type Grid = HashMap<Position, Num>;

const UP: Position = Position::new(0, -1);
const LEFT: Position = Position::new(-1, 0);
const RIGHT: Position = Position::new(1, 0);
const DOWN: Position = Position::new(0, 1);

fn make_grid(n: u32) -> Grid {
    Grid::from_iter(iproduct!(0..n, 0..n).map(|(a, b)| {
        (
            Position::new(a.try_into().unwrap(), b.try_into().unwrap()),
            (n * b + a + 1).try_into().unwrap(),
        )
    }))
}
#[cached]
fn grid() -> Grid {
    make_grid(3)
}
// essentually global constant but can't have 0..3 in const

fn conv(c: char) -> Position {
    match c {
        'U' => UP,
        'D' => DOWN,
        'L' => LEFT,
        'R' => RIGHT,
        _ => unreachable!(),
    }
}

fn move_(mut position: Position, moves:&[Position]) -> Position {
    let g = grid();
    for m in moves {
        let p = position + m;
        if g.contains_key(&p) {
            position = p;
        }
    }
    position
}

fn parse_row(base: &str) -> Vec<Position> {
    base.chars().map(conv).collect()
}

fn parse(base: &str) -> Vec<Vec<Position>> {
    base.split_whitespace().map(parse_row).collect()
}

pub fn part1(base: &str) -> i32 {
    let g = grid();
    let data = parse(base);
    data.iter()
        .scan(Position::new(1, 1), |state, x| {
            *state = move_(*state, x);
            g.get(state)
        })
        .map(|d| char::from_digit(*d as u32, 10).unwrap())
        .collect::<String>()
        .parse::<i32>()
        .unwrap()
}

pub fn part2(base: &str) -> i32 {
    //let data = parse(base);

    todo!();
}
