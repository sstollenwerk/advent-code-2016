use std::collections::HashMap;
use std::str::FromStr;

use crate::day23::Instruction::{Cpy, Dec, Inc, Jnz, Tgl};

type Num = i32;
type Register = char;

type Registers = HashMap<Register, Num>;

#[derive(Debug, Eq, PartialEq, Clone, Hash, Ord, PartialOrd, Copy)]
enum V {
    N(Num),
    C(Register),
}

#[derive(Debug, Eq, PartialEq, Clone, Hash, Ord, PartialOrd, Copy)]
enum Instruction {
    Cpy(V, V),
    Inc(V),
    Dec(V),
    Jnz(V, V),
    Tgl(V),
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(input: &str) -> Result<Instruction, Self::Err> {
        let mut vals = input.split_whitespace();
        let a = vals.next().unwrap();
        let b = (vals.next().unwrap());
        let c = vals.next();
        match a {
            "cpy" => Ok(Instruction::Cpy(as_v(b), as_v(c.unwrap()))),
            "inc" => Ok(Instruction::Inc(as_v(b))),
            "dec" => Ok(Instruction::Dec(as_v(b))),
            "jnz" => Ok(Instruction::Jnz(as_v(b), as_v(c.unwrap()))),
            "tgl" => Ok(Instruction::Tgl(as_v(b))),

            _ => Err(()),
        }
    }
}

fn as_v(b: &str) -> V {
    b.parse::<Num>().map_or_else(|_| V::C(first_char(b)), V::N)
}

fn first_char(s: &str) -> char {
    s.chars().next().unwrap()
}

fn parse(s: &str) -> Vec<Instruction> {
    s.trim()
        .lines()
        .map(|c| Instruction::from_str(c).unwrap())
        .collect()
}

fn run_code(mut registers: Registers, mut code: Vec<Instruction>) -> Registers {
    fn get_val(r: &V, reg: &Registers) -> Num {
        match r {
            V::N(n) => *n,
            V::C(c) => *reg.get(c).map(|c| c.to_owned()).get_or_insert(0),
        }
    }

    let mut i: usize = 0;

    while let Some(c) = code.get(i) {
        i += 1;

        match c {
            Cpy(v, r_) => {
                if let V::C(r) = r_ {
                    registers.insert(*r, get_val(v, &registers));
                };
            }
            Inc(r_) => {
                if let V::C(r) = r_ {
                    *registers.entry(*r).or_default() += 1
                }
            }
            Dec(r_) => {
                if let V::C(r) = r_ {
                    *registers.entry(*r).or_default() -= 1
                }
            }
            Jnz(v1, v2) => {
                let a = get_val(v1, &registers);
                let b = get_val(v2, &registers);
                if a != 0 {
                    i = ((i as i32) + (b - 1)) as usize;
                }
            }
            Instruction::Tgl(v) => {
                let p = (i - 1 + get_val(v, &registers) as usize);
                if let Some(comm) = (code.get(p)) {
                    code[p] = toggle(comm)
                }
            }
        };
    }
    registers
}

fn toggle(i: &Instruction) -> Instruction {
    match i {
        Cpy(a, b) => Jnz(*a, *b),
        Jnz(a, b) => Cpy(*a, *b),

        Inc(a) => Dec(*a),
        Dec(a) => Inc(*a),
        Tgl(a) => Inc(*a),
    }
}

pub fn part1(s: &str) -> Num {
    let r = parse(s);
    let res = run_code(Registers::from([('a', 7)]), r);
    println!("{:?}", &res);

    *res.get(&'a').unwrap()
}
pub fn part2(s: &str) -> Num {
    let r = parse(s);
    let res = run_code(Registers::from([('a', 12)]), r);
    println!("{:?}", &res);

    *res.get(&'a').unwrap()
}
