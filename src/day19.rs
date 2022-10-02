type Num = u32;

fn parse(s: &str) -> Num {
    s.parse().unwrap()
}

fn josephus2(n: Num) -> Num {
    let k = (n as f64).log2().floor() as Num;
    let m = (2 as Num).pow(k);
    assert!(n >= m && n < (2 * m));
    let l = n - m;
    2 * l + 1
}

pub fn part1(s: &str) -> Num {
    josephus2(parse(s))
}

pub fn part2(s: &str) -> Num {
    let r = parse(s);
    todo!();
}
