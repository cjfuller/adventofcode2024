use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

pub type Rules = HashMap<u32, HashSet<u32>>;

pub fn parse_rules(inp: &str) -> Rules {
    let mut output: Rules = HashMap::new();
    inp.lines()
        .map(|it| {
            it.split("|")
                .map(|it| it.parse().unwrap())
                .collect::<Vec<_>>()
        })
        .for_each(|it| {
            output
                .entry(it[1])
                .and_modify(|s| {
                    s.insert(it[0]);
                })
                .or_insert_with(|| HashSet::from([it[0]]));
        });

    output
}

fn rules() -> Rules {
    parse_rules(&std::fs::read_to_string("./inputs/day5_rules.txt").unwrap())
}

struct Update(Vec<u32>);

impl Update {
    pub fn mid(&self) -> u32 {
        self.0[self.0.len() / 2]
    }

    pub fn sorted(&self, rules: &Rules) -> Update {
        let mut new_values = self.0.clone();
        new_values.sort_by(|a, b| {
            if rules.get(b).map(|it| it.contains(a)).unwrap_or_default() {
                Ordering::Less
            } else if rules.get(a).map(|it| it.contains(b)).unwrap_or_default() {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        });
        Update(new_values)
    }
}

fn parse_updates(inp: &str) -> Vec<Update> {
    inp.lines()
        .filter(|it| !it.is_empty())
        .map(|l| l.split(",").map(|it| it.parse().unwrap()))
        .map(|it| Update(it.collect()))
        .collect()
}

fn updates() -> Vec<Update> {
    parse_updates(&std::fs::read_to_string("./inputs/day5_updates.txt").unwrap())
}

fn part1() -> u32 {
    updates()
        .into_iter()
        .filter(|it| it.sorted(&rules()).0 == it.0)
        .map(|it| it.mid())
        .sum()
}

fn part2() -> u32 {
    let r = rules();
    updates()
        .into_iter()
        .filter(|it| it.sorted(&r).0 != it.0)
        .map(|it| it.sorted(&r).mid())
        .sum()
}

fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}
