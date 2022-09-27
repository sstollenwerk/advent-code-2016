use num_complex::Complex;

type Num = i32;

type Position = Complex<Num>;

type Delta = (Position, Num);
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

pub fn part1(base: &String) -> i32 {
    let data = parse(base);

    let (dir, pos) = (NORTH, ORIGIN);
    let final_position = data.iter().fold((dir, pos), |(d, p), (rot, delta)| {
        ((d * rot), p + (d * rot).scale(*delta))
    });

    final_position.1.l1_norm()
}

pub fn part2(base: &String) -> i32 {
    let data = parse(base);

    todo!();
}
