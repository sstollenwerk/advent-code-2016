use std::collections::HashSet;

use itertools::iproduct;
use num_complex::Complex;

type Num = i32;

type Position = Complex<Num>;

type Pixels = HashSet<Position>;

//const TOPLEFT: Position = Position::new(0, 0);
const DOWN: Position = Position::new(0, 1);
const RIGHT: Position = Position::new(1, 0);

const SIZE: Position = Position::new(50, 6);

fn contain(start: &Position, window: &Position) -> Position {
    let a = start.re % window.re;
    let b = start.im % window.im;
    Position::new(a, b)
}

fn command(mut base: Pixels, c: &str, size: Position) -> Pixels {
    let binding = c.replace("x=", "").replace("y=", "");
    let w: Vec<_> = binding.split_whitespace().collect();
    if w[0] == "rect" {
        let items: Vec<_> = w[1].split('x').map(|s| s.parse::<i32>().unwrap()).collect();
        let to_add = iproduct!(0..items[0], 0..items[1]).map(|(a, b)| Position::new(a, b));
        base.extend(to_add);
    } else if w[1] == "column" {
        let items: Vec<_> = [w[2], w[4]]
            .iter()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        let to_get: Pixels = (0..50)
            .map(|n| RIGHT.scale(items[0]) + DOWN.scale(n))
            .collect();
        let in_both = base.intersection(&to_get).copied().collect();
        base = base.difference(&in_both).copied().collect();
        let shifted = in_both.iter().map(|c| c + DOWN.scale(items[1]));
        base.extend(shifted);
    } else if w[1] == "row" {
        let items: Vec<_> = [w[2], w[4]]
            .iter()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        let to_get: Pixels = (0..50)
            .map(|n| DOWN.scale(items[0]) + RIGHT.scale(n))
            .collect();
        let in_both = base.intersection(&to_get).copied().collect();
        base = base.difference(&in_both).copied().collect();
        let shifted = in_both.iter().map(|c| c + RIGHT.scale(items[1]));
        base.extend(shifted);
    }
    base = base.iter().map(|c| contain(c, &size)).collect();
    // display(&base, &size);
    // For debugging
    base
}

fn display(p: &Pixels, size: &Position) {
    let mut vals: Vec<String> = Vec::new();

    for r in (0..size.im) {
        let posses = (0..size.re)
            .map(|k| Position::new(k, r))
            .map(|c| p.contains(&c))
            .map(|b| if b { '#' } else { ' ' })
            .collect();
        vals.push(posses)
    }
    for r in vals {
        println!("{:?}", &r);
    }

    println!();
}

pub fn part1(s: &str) -> usize {
    let size = SIZE;
    //  let size = Position::new(7, 3);
    // for debugging
    s.lines()
        .fold(Pixels::new(), |acc, x| command(acc, x, size))
        .len()
}

pub fn part2(s: &str) {
    let size = SIZE;
    let res = s
        .lines()
        .fold(Pixels::new(), |acc, x| command(acc, x, size));

    display(&res, &size)
}
