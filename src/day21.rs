use std::str::FromStr;

use crate::day21::Instruction::*;

use itertools::Itertools;

#[derive(Debug, Eq, PartialEq, Clone, Hash, Ord, PartialOrd, Copy)]
enum Instruction {
    SwapPos(usize, usize),
    SwapLet(char, char),
    RotateLeft(usize),
    RotateRight(usize),
    RotateLet(char),
    Reverse(usize, usize),
    Move(usize, usize),
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(input: &str) -> Result<Instruction, Self::Err> {
        let vals: Vec<_> = input.split_whitespace().collect();
        let a = vals[0];
        let b = vals[1];
        match (a, b) {
            ("swap", "position") => Ok(Instruction::SwapPos(as_num(vals[2]), as_num(vals[5]))),
            ("swap", "letter") => Ok(Instruction::SwapLet(
                first_char(vals[2]),
                first_char(vals[5]),
            )),
            ("rotate", "left") => Ok(Instruction::RotateLeft(as_num(vals[2]))),
            ("rotate", "right") => Ok(Instruction::RotateRight(as_num(vals[2]))),
            ("rotate", "based") => Ok(Instruction::RotateLet(first_char(vals[6]))),
            ("reverse", "positions") => Ok(Instruction::Reverse(as_num(vals[2]), as_num(vals[4]))),
            ("move", "position") => Ok(Instruction::Move(as_num(vals[2]), as_num(vals[5]))),

            _ => Err(()),
        }
    }
}

fn first_char(s: &str) -> char {
    s.chars().next().unwrap()
}

fn as_num(s: &str) -> usize {
    s.parse().unwrap()
}

fn parse(s: &str) -> Vec<Instruction> {
    s.trim()
        .lines()
        .map(|c| Instruction::from_str(c).unwrap())
        .collect()
}

fn sing_command(mut r: Vec<char>, instr_: &Instruction) -> Vec<char> {
    let instr = instr_.to_owned();
    //  println!("{:?}", &instr);

    match instr {
        SwapPos(a, b) => {
            r.swap(a, b);
            r
        }
        Reverse(a, b) => {
            let mut r_ = r.to_owned();
            let p1 = &r[..a];
            let p2 = &r[b + 1..];
            let mid = &mut r_[a..b + 1];
            mid.reverse();
            [p1, mid, p2].concat()
        }
        RotateLeft(n) => {
            r.rotate_left(n);
            r
        }
        RotateRight(n) => {
            r.rotate_right(n);
            r
        }
        Move(a, b) => {
            let k = r.remove(a);
            r.insert(b, k);
            r
        }
        _ => todo!(),
    }
}

fn command(mut r: Vec<char>, instrs: &Vec<Instruction>) -> Vec<char> {
    //  println!("{:?}", &r);
    for i in instrs {
        r = match i {
            SwapLet(c, d) => {
                let a_: usize = index(&r, c).unwrap();
                let b_: usize = index(&r, d).unwrap();
                let a = a_.min(b_);
                let b = a_.max(b_);
                sing_command(r, &SwapPos(a, b))
            }
            RotateLet(c) => {
                let mut n: usize = index(&r, c).unwrap();
                if n >= 4 {
                    n += 1;
                }
                n += 1;
                let le = r.len();
                sing_command(r, &RotateRight(n % le))
            }
            x => sing_command(r, x),
        };
        //    println!("{:?}", &r);
    }
    r
}

fn index<T: Eq + PartialEq>(xs: &[T], target: &T) -> Option<usize> {
    xs.iter().position(|item| item == target)
}

pub fn part1(s: &str) -> String {
    let r = parse(s);

    let input: Vec<char> = "abcdefgh".chars().collect();
    command(input, &r).into_iter().collect()
}
pub fn part2(s: &str) -> String {
    // brute force takes unnoticeable amount of time.
    let r = parse(s);

    let desired: Vec<char> = "fbgdceah".chars().collect();
    let mut posses = desired.clone().into_iter().permutations(desired.len());
    posses
        .find(|i| command(i.to_vec(), &r) == desired)
        .unwrap()
        .into_iter()
        .collect()
}
