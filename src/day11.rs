use std::collections::HashSet;
use std::str::FromStr;

use itertools::{Either, Itertools};

use pathfinding::directed::dijkstra::dijkstra;

#[derive(Debug, Eq, PartialEq, Clone, Hash, Ord, PartialOrd)]
enum Object {
    Microchip(String),
    Generator(String),
}

impl FromStr for Object {
    type Err = ();

    fn from_str(input: &str) -> Result<Object, Self::Err> {
        let mut vals = input.split_whitespace();
        let a = vals.next().unwrap().to_string();
        let b = (vals.next().unwrap());
        match b {
            "microchip" => Ok(Object::Microchip(a)),
            "generator" => Ok(Object::Generator(a)),
            _ => Err(()),
        }
    }
}

type Floor = Vec<Object>;
// needs to be vec not hashset because hashset doesn't implement hash.

type Building = Vec<Floor>;

type State = (usize, Building);

type Edge = (State, u32);

fn floor_safe(f: &Floor) -> bool {
    // if a generator in a building, every Microchip must have a matching generator

    let (generators, microchips): (HashSet<String>, HashSet<String>) =
        f.iter().partition_map(|r| match r {
            Object::Generator(v) => Either::Left(v.clone()),
            Object::Microchip(v) => Either::Right(v.clone()),
        });

    generators.is_empty() || (microchips.is_subset(&generators))
}

fn floors_safe(s: &State) -> bool {
    s.1.iter().all(floor_safe)
}

fn success(s: &State) -> bool {
    s.1.iter().rev().skip(1).all(|r| r.is_empty())
}

fn heuristic(s: &State) -> u32 {

 s.1.iter().rev().enumerate().map(|(d, r)| ((d*r.len() )  /2) as u32 ).sum() 
}

fn neighbours1(s: &State) -> Vec<Edge> {
    let (f, b) = s;
    let posses = if f != &0 {
        vec![f - 1, f + 1]
    } else {
        vec![f + 1]
    };
    let nexts = posses.iter().filter(|&&n| (n >= 0) && (n < b.len()));
    let current: HashSet<Object> = b[*f].clone().into_iter().collect();
    let subsets: Vec<HashSet<Object>> = (1..=(2.min(current.len())))
        .flat_map(|n| current.iter().combinations(n))
        .map(|n| {
            n.into_iter()
                .map(|c| c.to_owned())
                .collect::<HashSet<Object>>()
        })
        .collect();

    let mut res: Vec<Edge> = Vec::new();
    for next_ in nexts {
        let next_floor = b[*next_].clone();

        for s_ in subsets.iter() {
            let g = s_.clone();

            let mut b_ = b.clone();
            let mut start: Vec<Object> = current.difference(&g).cloned().to_owned().collect();
            start.sort();
            b_[*f] = start;

            let mut end: Vec<Object> = next_floor
                .clone()
                .into_iter()
                .collect::<HashSet<Object>>()
                .union(&g)
                .cloned()
                .to_owned()
                .collect();
            end.sort();

            b_[*next_] = end;

            res.push(((*next_, b_), 1));
        }
    }

    res.retain(|(f, _)| floors_safe(f));
    res
}

fn parse_row(s: &str) -> Floor {
    if s.contains("nothing relevant") {
        return Floor::new();
    }
    let binding = s
        .replace("and ", "")
        .replace(',', "")
        .replace('.', "")
        .replace("-compatible", "");
    binding
        .split("a ")
        .skip(1)
        .map(|g| Object::from_str(g).unwrap())
        .collect()
}

fn parse(s: &str) -> Building {
    s.trim().lines().map(parse_row).collect()
}

pub fn part1(s: &str) -> u32 {
    let r = parse(s);
    println!("{:?}", &r);
    let (path, cost) = dijkstra(&(0, r), neighbours1, success).unwrap();
    cost
}
pub fn part2(s: &str) -> u32 {
    let extra = vec![
        Object::Microchip("elerium".to_string()),
        Object::Generator("elerium".to_string()),
        Object::Microchip("dilithium".to_string()),
        Object::Generator("dilithium".to_string()),
    ];
    let mut r = parse(s);
    r[0].extend(extra);
    println!("{:?}", &r);

    let (path, cost) = dijkstra(&(0, r), neighbours1, success).unwrap();
    cost
}
