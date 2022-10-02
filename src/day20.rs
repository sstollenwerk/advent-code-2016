use std::iter;

type Num = u64;

type Group = (Num, Num);

const LIMIT: Num = 4294967296;

fn parse_row(s: &str) -> Group {
    let mut parts = s.split('-').map(|c| c.parse().unwrap());

    (parts.next().unwrap(), parts.next().unwrap() + 1)
    // going with Half Closed Interval, [a, b)
}

fn merge_rows(xs_: &[Group]) -> Vec<Group> {
    let mut xs: Vec<Option<Group>> = xs_.iter().map(|x| Some(*x)).collect();
    xs.sort_by_key(|x| x.unwrap().0);

    for i in 0..(xs.len() - 1) {
        let (a, b) = xs[i].unwrap();
        let (c, d) = xs[i + 1].unwrap();
        if b >= c {
            xs[i + 1] = Some((a.min(c), d.max(b)));
            xs[i] = None;
        }
    }
    xs.into_iter().flatten().collect()
}

fn parse(s: &str) -> Vec<Group> {
    s.lines().map(parse_row).collect()
}

fn permitted(xs: &[Group]) -> Num {
    let banned = merge_rows(xs);
    let last = iter::once((banned.last().unwrap().1, LIMIT));
    let allowed = banned.windows(2).map(|g| (g[0].1, g[1].0)).chain(last);
    allowed.map(|(a, b)| b - a).sum()
}

pub fn part1(s: &str) -> Num {
    let vals = parse(s);
    let res = merge_rows(&vals);
    assert!(res == merge_rows(&res));
    res[0].1
}

pub fn part2(s: &str) -> Num {
    let vals = parse(s);
    permitted(&vals)
}
