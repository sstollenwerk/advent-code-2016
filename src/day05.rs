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

    println!("{:?}", vec![0; 5]);
    println!("{:?}", &find_hash(&"abc3231929")[0..5] == vec![0; 5]);

    let items = (0..);

    let posses = items
        .map(|n| (b.clone()) + &(n.to_string()))
        .map(|s| find_hash(&s))
        .filter(|c| &c[0..5] == vec![0; 5])
        .take(8)
        .map(|r| char::from_digit(r[5].into(), 16).unwrap())
        .collect();

    posses
}

pub fn part2(base: &str) -> u32 {
    todo!();
}
