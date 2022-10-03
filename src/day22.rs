use std::collections::HashMap;

use itertools::iproduct;
use num_complex::Complex;

type Position = Complex<i32>;

#[derive(Debug, Clone, Eq, PartialEq)]
struct Node {
    place: Position,
    size: u32,
    used: u32,
    avail: u32,
    use_per: u32,
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
            .replace('T', "")
            .replace('%', "")
            .replace('%', "")
            .replace("/dev/grid/node-", "")
            .replace('x', "")
            .replace('y', "")
            .replace('-', " ")
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        let p = Position::new(parts[0] as i32, parts[1] as i32);

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
    println!("{:?}", &r[50]);

    all_fit(&r)
}

pub fn part2(s: &str) -> String {
    let r = parse(s);

    todo!();
}
