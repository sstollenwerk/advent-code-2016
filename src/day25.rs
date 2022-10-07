use std::collections::HashMap;
use std::str::FromStr;

use crate::day25::Instruction::{Add, Cpy, Dec, Inc, Jnz, Mul, Out};
use crate::day25::V::{C, N};

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
    Cpy(V, Register),
    Inc(Register),
    Dec(Register),
    Jnz(V, V),
    Out(V),
    Add(V, Register),
    Mul(V, Register),
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(input: &str) -> Result<Instruction, Self::Err> {
        let mut vals = input.split_whitespace();
        let a = vals.next().unwrap();
        let b = (vals.next().unwrap());
        let c = vals.next();
        match a {
            "cpy" => Ok(Instruction::Cpy(as_v(b), first_char(c.unwrap()))),
            "inc" => Ok(Instruction::Inc(first_char(b))),
            "dec" => Ok(Instruction::Dec(first_char(b))),
            "jnz" => Ok(Instruction::Jnz(as_v(b), as_v(c.unwrap()))),
            "out" => Ok(Instruction::Out(as_v(b))),

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

fn run_code(mut registers: Registers, code: &[Instruction]) -> bool {
    fn get_val(r: &V, reg: &Registers) -> Num {
        match r {
            V::N(n) => *n,
            V::C(c) => *reg.get(c).map(|c| c.to_owned()).get_or_insert(0),
        }
    }

    let mut i: usize = 0;

    let mut res = Vec::new();

    let desired: Vec<Num> = (0..100).map(|x| x % 2).collect();

    while let Some(c) = code.get(i) {
        i += 1;

        match c {
            Cpy(v, r) => {
                registers.insert(*r, get_val(v, &registers));
            }
            Inc(r) => *registers.entry(*r).or_default() += 1,
            Dec(r) => *registers.entry(*r).or_default() -= 1,
            Jnz(v1, v2) => {
                let a = get_val(v1, &registers);
                let b = get_val(v2, &registers);
                if a != 0 {
                    i = ((i as i32) + (b - 1)) as usize;
                }
            }
            Out(v) => res.push(get_val(v, &registers)),

            Add(v, r) => *registers.entry(*r).or_default() += get_val(v, &registers),

            Mul(v, r) => *registers.entry(*r).or_default() *= get_val(v, &registers),
        };
        if res.len() == desired.len() {
            return res == desired;
        }
    }
    false
}

pub fn part1(s: &str) -> Num {
    let mut r = parse(s);

    let replace = [
        Mul(C('b'), ('c')),
        Add(C('c'), ('d')),
        Cpy(N(0), ('b')),
        Cpy(N(0), ('c')),
    ];

    r[3..(replace.len() + 3)].copy_from_slice(&replace[..]);

    (0..)
        .find(|x| run_code(HashMap::from([('a', *x)]), &r))
        .unwrap()
}
pub fn part2(_s: &str) -> Num {
    todo!();
}
