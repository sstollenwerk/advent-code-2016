use counter::Counter;

use csr::Caesar;

#[derive(Debug, Clone, Eq, PartialEq)]
struct Room {
    name: Vec<String>,
    checksum: String,
    id: u32,
}

impl Room {
    fn new(name: Vec<String>, checksum: String, id: u32) -> Room {
        Room { name, checksum, id }
    }

    fn from_raw(base: &str) -> Room {
        let parts: Vec<&str> = base.split('[').collect();
        let checksum = parts[1].strip_suffix(']').unwrap().to_string();
        let section = parts[0]
            .split('-')
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        let (id_, vals) = section.split_last().unwrap();
        let id = id_.parse().unwrap();
        Room::new(vals.to_vec(), checksum, id)
    }
}

fn to_checksum(r: &Room) -> String {
    r.name
        .iter()
        .flat_map(|s| s.chars())
        .collect::<Counter<_>>()
        .k_most_common_ordered(5)
        .iter()
        .map(|x| x.0)
        .collect()
}

fn matches_checksum(r: &Room) -> bool {
    to_checksum(r) == r.checksum
}

fn decrypt(r: &Room) -> String {
    let base = r.name.join(" ");

    let c = Caesar::new(r.id % 26);
    c.encrypt(base)
}

fn parse(base: &str) -> Vec<Room> {
    base.lines().map(Room::from_raw).collect()
}

pub fn part1(base: &str) -> u32 {
    let data = parse(base);

    data.iter()
        .filter(|r| matches_checksum(r))
        .map(|x| x.id)
        .sum()
}

pub fn part2(base: &str) -> u32 {
    let fake = Room::from_raw("qzmt-zixmtkozy-ivhz-343[abc]");
    println!("{:?}", &decrypt(&fake));

    let desired = "northpole object storage".to_string();

    let data = parse(base);
    data.iter()
        .filter(|r| matches_checksum(r))
        .find(|r| decrypt(r) == desired)
        .unwrap()
        .id
}
