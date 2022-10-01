// want i32.rem_euclid not %.

type Num = i32;
// fairly certain answer <= product( number of positions)
// ceil(log2(prod)) = 19.
// small enough.

type Position = i32;
type Disc = (Position, Position);

fn optimal_time(discs: &[Disc]) -> Num {
    let mut prod = 1;
    // all items in input are coprime - lcm == prod
    let mut time = 0;

    for (i, &(gaps, current_)) in discs.iter().enumerate() {
        let desired = (gaps - (i as Num) - 1).rem_euclid(gaps);
        let mut current = (current_ + time).rem_euclid(gaps);
        while current != desired {
            current = (current + prod).rem_euclid(gaps);
            time += prod;
        }
        prod *= gaps;
    }

    time
}

fn parse_row(s: &str) -> Disc {
    let binding = s.replace('.', "");
    let parts: Vec<_> = binding.split_whitespace().collect();
    let vals = vec![parts[3], parts[11]];
    let r: Vec<Num> = vals.iter().map(|c| c.parse().unwrap()).collect();
    (r[0], r[1])
}

fn parse(s: &str) -> Vec<Disc> {
    s.lines().map(parse_row).collect()
}
pub fn part1(s: &str) -> Num {
    let vals = parse(s);
    optimal_time(&vals)
}

pub fn part2(s: &str) -> Num {
    todo!();
}
