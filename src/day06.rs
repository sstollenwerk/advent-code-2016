use counter::Counter;

fn transpose<T>(v: &Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    //copied and slightly modified from
    //  https://stackoverflow.com/a/64499219
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}

pub fn part1(base: &str) -> String {
    let columns = transpose(
        &(base
            .trim()
            .to_string()
            .lines()
            .map(|r| r.chars().collect::<Vec<_>>())
            .collect()),
    );
    columns
        .iter()
        .map(|r| r.iter().collect::<Counter<_>>().k_most_common_ordered(1)[0].0)
        .collect()
}
pub fn part2(base: &str) -> String {
    let columns = transpose(
        &(base
            .trim()
            .to_string()
            .lines()
            .map(|r| r.chars().collect::<Vec<_>>())
            .collect()),
    );
    columns
        .iter()
        .map(|r| {
            r.iter()
                .collect::<Counter<_>>()
                .most_common_ordered()
                .last()
                .unwrap()
                .0
        })
        .collect()
}
