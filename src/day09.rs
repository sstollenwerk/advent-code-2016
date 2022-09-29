fn find_size(s: &str) -> usize {
    // in input all chars are upper outsize of brackets so I don't have to find better parsing
    if let Some(p) = s.find('x') {
        let a = s.find('(').unwrap();
        let b = s.find(')').unwrap();
        assert!((a < p) && (p < b));

        let mut part = s[(a + 1)..b]
            .split('x')
            .map(|c| c.parse::<usize>().unwrap());
        let to_see = part.next().unwrap();
        let amt = part.next().unwrap();

        a + (to_see * amt) + find_size(&s[(b + 1 + to_see)..])
    } else {
        s.len()
    }
}

pub fn part1(s: &str) -> usize {
    find_size(s.trim())
}

pub fn part2(s: &str) -> usize {
    todo!();
}
