use crate::helper::all_eq;

type Dragon = Vec<bool>;

fn dragon_step(d: &Dragon) -> Dragon {
    let mut a: Dragon = d.clone();
    let mut b: Dragon = a.iter().map(|b| !b).collect();
    b.reverse();
    a.push(false);
    a.append(&mut b);
    a
}

fn parse(s: &str) -> Dragon {
    s.chars().map(|c| c == '1').collect()
}

fn to_size(mut d: Dragon, size: usize) -> Dragon {
    while d.len() < size {
        d = dragon_step(&d)
    }
    d[0..size].to_vec()
}

fn check_step(d: &Dragon) -> Dragon {
    d.chunks_exact(2).map(all_eq).collect()
}

fn checksum(mut d: Dragon) -> Dragon {
    while (d.len() % 2) == 0 {
        d = check_step(&d);
    }
    d
}

fn as_disp(d: &Dragon) -> String {
    d.iter().map(|b| if *b { '1' } else { '0' }).collect()
}

pub fn part1(s: &str) -> String {
    let vals = parse(s);
    let desired_length = 272;
    let res = checksum(to_size(vals, desired_length));
    println!("{:?}", &res);

    as_disp(&res)
}

pub fn part2(s: &str) -> String {
    let vals = parse(s);
    let desired_length = 35651584;
    // ~35 million
    // 2^21 * 17
    // brute force - 264 milliseconds to run.
    // wow
    let res = checksum(to_size(vals, desired_length));
    println!("{:?}", &res);

    as_disp(&res)
}
