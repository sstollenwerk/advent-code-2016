use std::collections::HashMap;

type Num = u32;

#[derive(Debug, Eq, PartialEq, Clone, Hash, Copy)]
enum ObjType {
    Bot,
    Output,
}

type Object = (Num, ObjType);

type Owns = HashMap<Object, Vec<Num>>;

type Commands = HashMap<Object, (Object, Object)>;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Instruction {
    Base(Num, Object),
    Shift(Object, Object, Object),
}

fn as_obj(s: &str) -> ObjType {
    match s {
        "bot" => ObjType::Bot,
        "output" => ObjType::Output,
        _ => unreachable!(),
    }
}

fn parse_row(s: &str) -> Instruction {
    let w: Vec<_> = s.split_whitespace().collect();

    match w[0] {
        "value" => Instruction::Base(w[1].parse().unwrap(), (w[5].parse().unwrap(), as_obj(w[4]))),

        "bot" => Instruction::Shift(
            (w[1].parse().unwrap(), as_obj(w[0])),
            (w[6].parse().unwrap(), as_obj(w[5])),
            (w[11].parse().unwrap(), as_obj(w[10])),
        ),

        _ => unreachable!(),
    }
}

fn parse(s: &str) -> (Owns, Commands) {
    let mut holds: Owns = Owns::new();
    let mut commands: Commands = Commands::new();
    let vals = s.trim().lines().map(parse_row);
    for c in vals {
        match c {
            Instruction::Base(item, bot) => {
                let v = holds.entry(bot).or_default();

                v.push(item);
            }
            Instruction::Shift(a, b, c) => {
                commands.insert(a, (b, c));
            }
        };
    }
    (holds, commands)
}

fn bot_emulate(mut holds: Owns, commands: &Commands) -> Owns {
    while let Some(b) = holds
        .iter()
        .filter(|(k, v)| (v.len() >= 2) && (k.1 == ObjType::Bot))
        .map(|t| t.0)
        .next()
    {
        let b_ = b.clone();
        let mut vals = holds.get(b).unwrap().clone();
        assert!(vals.len() == 2);
        vals.sort();
        if vals == vec![17, 61] {
            println!("{:?}", &b);
        }

        let com = commands.get(b).unwrap();
        holds.entry(com.1).or_default().push(vals.pop().unwrap());

        holds.entry(com.0).or_default().push(vals.pop().unwrap());

        holds.insert(b_, vals);
    }

    holds
}

pub fn part1(s: &str) {
    let (holds, commands) = parse(s);
    bot_emulate(holds, &commands);
}

pub fn part2(s: &str) -> Num {
    let (holds, commands) = parse(s);
    let res = bot_emulate(holds, &commands);

    (0..=2)
        .flat_map(|n| res.get(&(n, ObjType::Output)).unwrap())
        .product()
}
