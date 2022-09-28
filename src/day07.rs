use std::collections::HashSet;

fn is_abba(s: &Vec<char>) -> bool {
    s.windows(4).any(base_abba)
}

fn base_abba(s_: &[char]) -> bool {
    let mut s = s_.to_vec();
    let k = s.len() / 2;
    let a = &s_[0..k];
    let b = &mut s[k..];
    b.reverse();
    (a[0] != a[1]) && (a == b)
}
fn is_tls(xs: &Vec<Vec<char>>) -> bool {
    let a = xs.iter().step_by(2).any(is_abba);
    let b = xs[1..].iter().step_by(2).any(is_abba);
    a && (!b)
}

fn base_aba(s: &Vec<char>) -> Vec<Vec<char>> {
    s.to_owned()
        .windows(3)
        .filter(|xs| (xs[0] == xs[2]) && (xs[0] != xs[1]))
        .map(|c| c.to_vec())
        .collect()
}

fn is_ssl(xs: &Vec<Vec<char>>) -> bool {
    let a: HashSet<_> = xs
        .iter()
        .step_by(2)
        .flat_map(base_aba)
        .map(|c| to_bab(&c))
        .collect();
    let b: HashSet<_> = xs[1..].iter().step_by(2).flat_map(base_aba).collect();

    !(a.is_disjoint(&b))
}

fn to_bab(aba: &[char]) -> Vec<char> {
    vec![aba[1], aba[0], aba[1]]
}

fn parse(s: &str) -> Vec<Vec<Vec<char>>> {
    let parts = s.replace(']', "[");

    parts
        .lines()
        .map(|r| r.split('[').map(|c| c.chars().collect()).collect())
        .collect()
}

pub fn part1(s: &str) -> usize {
    parse(s).into_iter().filter(is_tls).count()
}

pub fn part2(s: &str) -> usize {
    parse(s).into_iter().filter(is_ssl).count()
}
