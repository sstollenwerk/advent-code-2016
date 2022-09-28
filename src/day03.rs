type Num = u32;

fn valid_triangle(tri_: &Vec<Num>) -> bool {
    let mut tri = tri_.to_owned();
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

pub fn part1(base: &str) -> usize {
    let data = parse(base);
    data.into_iter().filter(valid_triangle).count()
}

pub fn part2(base: &str) -> usize {
    let data = parse(base);

    todo!();
}
