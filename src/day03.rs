type Num = u32;

fn valid_triangle(tri_: &[Num]) -> bool {
    let mut tri = tri_.to_vec();
    tri.sort_unstable();
    assert!(tri.len() == 3);
    (tri[0] + tri[1]) > tri[2]
}

fn parse(base: &str) -> Vec<Vec<Num>> {
    base.trim()
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect()
        })
        .collect()
}

fn transpose_flatten<T: Copy>(vals: &[Vec<T>]) -> Vec<T> {
    let mut res = Vec::new();
    for i in 0..(vals[0].len()) {
        for v in vals {
            res.push(v[i]);
        }
    }
    res
}

pub fn part1(base: &str) -> usize {
    let data = parse(base);
    data.into_iter().filter(|c| valid_triangle(c)).count()
}

pub fn part2(base: &str) -> usize {
    let data = parse(base);
    transpose_flatten(&data)
        .chunks(3)
        .filter(|c| valid_triangle(c))
        .count()
}
