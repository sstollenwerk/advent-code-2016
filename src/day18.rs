// is rule 90 as far as I can tell

use std::iter;

type Cell = bool;
type Row = Vec<Cell>;

//const SAFE: bool = false;

fn rule90(step: &[Cell]) -> Cell {
    let posses: Vec<_> = vec![
        vec![true, true, false],
        vec![false, true, true],
        vec![true, false, false],
        vec![false, false, true],
    ];

    posses.contains(&step.to_vec())
}

fn step(row_: &Row) -> Row {
    let mut row = row_.to_owned();
    row.push(false);
    row.insert(0, false);
    row.windows(3).map(rule90).collect()
}

fn parse(s: &str) -> Row {
    s.trim().chars().map(|c| c == '^').collect()
}
pub fn part1(s: &str) -> usize {
    let row = parse(s);
    let steps = 40;
    println!("{:?}", &row);
    let mut state = row;
    // using iter::repeat_with as similar to haskell's Prelude.iterate
    let rows = iter::repeat_with(|| {
        let tmp = state.clone();
        state = step(&state);
        tmp
    })
    .take(steps);
    rows.flatten().filter(|x| !x).count()
}

pub fn part2(s: &str) -> usize {
    let row = parse(s);
    let steps = 400000;
    let mut state = row;
    let rows = iter::repeat_with(|| {
        let tmp = state.clone();
        state = step(&state);
        tmp
    })
    .take(steps);
    rows.flatten().filter(|x| !x).count()
}
