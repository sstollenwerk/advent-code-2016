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
fn josephus_half_ineff(n: Num) -> Num {
    let vals: Vec<Num> = (0..n).collect();

    josephus_brute_force(vals) + 1
}

fn josephus_brute_force<T: std::fmt::Debug + Copy>(mut vals: Vec<T>) -> T {
    let mut i: usize = 0;

    while vals.len() > 1 {
        let k = vals.len() / 2;
        let p = (i + k) % vals.len();
        vals.remove(p);
        if p > i {
            i += 1;
        }
        i %= vals.len();
    }
    vals[0]
}

fn josephus_half_optim_1(n: Num) -> Num {
    let mut vals: Vec<Num> = (0..n).collect();

    // will get indexerror with small vals. Fortunately, brute force works at that point
    while vals.len() > 10 {
        let k = ((vals.len() + 2) % 3);
        let b = vals.len() % 2;
        let pos = vals.len() / 2 - 1 - b;

        let mut a: Vec<Num> = vals[k..pos].iter().step_by(3).copied().collect();
        let mut b: Vec<Num> = vals[pos..].iter().step_by(3).copied().collect();
        if vals.len() % 6 == 1 {
            b[0] = vals[pos + 1];
        }
        a.extend(b);
        vals = a;
    }
    josephus_brute_force(vals) + 1
}
pub fn part1(s: &str) -> Num {
    josephus2(parse(s))
}

pub fn part2(s: &str) -> Num {
    let r = parse(s);
    // println!("{:?}", &josephus_half_ineff(128));
    // Did a lot of testing and fiddling with algorithm to get efficient version working.
    josephus_half_optim_1(r)
}
