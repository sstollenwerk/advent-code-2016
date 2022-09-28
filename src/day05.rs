//use hex_literal::hex;
use md5::{Digest, Md5};

fn as_u4(vals: &[u8]) -> Vec<u8> {
    vals.iter().flat_map(|k| [k / 16, k % 16]).collect()
}

fn find_hash(s: &str) -> Vec<u8> {
    let mut hasher = Md5::new();
    hasher.update(s);

    as_u4(&hasher.finalize())
}

pub fn part1(base: &str) -> String {
    let b = base.to_string();

    let items = (0..);

    items
        .map(|n| (b.clone()) + &(n.to_string()))
        .map(|s| find_hash(&s))
        .filter(|c| c[0..5] == vec![0; 5])
        .take(8)
        .map(|r| char::from_digit(r[5].into(), 16).unwrap())
        .collect()
}

pub fn part2(base: &str) -> String {
    let b = base.to_string();

    let mut res = vec![None; 8];

    let posses = (0..)
        .map(|n| (b.clone()) + &(n.to_string()))
        .map(|s| find_hash(&s))
        .filter(|c| c[0..5] == vec![0; 5]);

    for r in posses {
        let p = r[5] as usize;
        if ((p < res.len()) && (res[p].is_none())) {
            res[p] = char::from_digit(r[6].into(), 16);
            if res.iter().all(|c| c.is_some()) {
                break;
            }
        }
    }

    res.iter().map(|c| c.unwrap()).collect()
}
