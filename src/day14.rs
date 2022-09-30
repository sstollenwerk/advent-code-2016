use md5::{Digest, Md5};

use rayon::prelude::*;

type Num = u32;
type Hashed = Vec<char>;

fn as_u4(vals: &[u8]) -> Vec<u8> {
    vals.iter().flat_map(|k| [k / 16, k % 16]).collect()
}

fn as_s(vals: &[u8]) -> Hashed {
    let r = as_u4(vals);
    r.into_iter()
        .map(|n| char::from_digit(n as u32, 16).unwrap())
        .collect()
}

fn find_hash(r_: &Hashed, steps: Num) -> Hashed {
    let mut r = r_.to_vec();
    for _ in (0..steps) {
        r = base_hash(&r)
    }

    r
}

fn base_hash(s: &Hashed) -> Hashed {
    let r: String = s.iter().collect();
    let mut hasher = Md5::new();
    hasher.update(r);

    as_s(&hasher.finalize())
}

fn hasher(salt: &str, n: Num, steps: Num) -> Hashed {
    let s = salt.to_owned() + &(n.to_string());
    let r = s.chars().collect();
    find_hash(&r, steps)
}

fn all_eq<T: Eq>(xs: &[T]) -> bool {
    let first = xs.get(0);
    xs.iter().all(|i| Some(i) == first)
}

fn sequences<T: Eq + Clone>(n: usize, xs: &[T]) -> Vec<Vec<T>> {
    xs.windows(n)
        .filter(|w| all_eq(w))
        .map(|c| c.to_vec())
        .collect()
}

fn valid(n: Num, salt: &str, steps: Num) -> bool {
    let h = hasher(salt, n, steps);
    if let Some(s) = sequences(3, &h).get(0) {
        let a = s[0];
        let desired = vec![a; 5];

        let posses = ((n + 1)..(n + 1 + 1000))
            .into_par_iter()
            .map(|k| hasher(salt, k, steps));
        posses
            .into_par_iter()
            .flat_map(|p| sequences(5, &p))
            .any(|r| r == desired)
    } else {
        false
    }
}

pub fn part1(s: &str) -> Num {
    (0..).filter(|&n| valid(n, s, 1)).nth(63).unwrap()
}
pub fn part2(s: &str) -> Num {
    (0..).filter(|&n| valid(n, s, 2017)).nth(63).unwrap()
}
