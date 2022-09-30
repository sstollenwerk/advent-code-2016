use md5::{Digest, Md5};
type Num = u32;
type Hashed = Vec<u8>;

fn as_u4(vals: &[u8]) -> Hashed {
    vals.iter().flat_map(|k| [k / 16, k % 16]).collect()
}

fn find_hash(s: &str) -> Hashed {
    let mut hasher = Md5::new();
    hasher.update(s);

    as_u4(&hasher.finalize())
}

fn hasher(salt: &str, n: Num) -> Hashed {
    find_hash(&(salt.to_owned() + &(n.to_string())))
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

fn valid(n: Num, salt: &str) -> bool {
    let h = hasher(salt, n);
    if let Some(s) = sequences(3, &h).get(0) {
        let a = s[0];
        let desired = vec![a; 5];

        let posses = ((n + 1)..(n + 1 + 1000)).map(|k| hasher(salt, k));
        posses.flat_map(|p| sequences(5, &p)).any(|r| r == desired)
    } else {
        false
    }
}

pub fn part1(s: &str) -> Num {
    (1..).filter(|&n| valid(n, s)).nth(63).unwrap()
}
pub fn part2(s: &str) -> Num {
    todo!();
}
