use std::collections::HashMap;

fn inputs() -> String {
    std::fs::read_to_string("./inputs/day1.txt").unwrap()
}

fn parse_inputs(inp: &str) -> (Vec<i32>, Vec<i32>) {
    let mut first = vec![];
    let mut second = vec![];

    inp.lines()
        .map(|l| l.split(" ").filter(|it| !it.is_empty()))
        .for_each(|s| {
            let s = s.collect::<Vec<_>>();
            first.push(s[0].parse::<i32>().unwrap());
            second.push(s[1].parse::<i32>().unwrap());
        });

    (first, second)
}

fn part1(inp: &str) -> i32 {
    let (mut first, mut second) = parse_inputs(inp);
    first.sort();
    second.sort();
    first
        .into_iter()
        .zip(second)
        .map(|(a, b)| (a - b).abs())
        .sum()
}

fn sim_score(first: &[i32], second: &[i32]) -> i32 {
    let mut counts = HashMap::new();
    second.iter().for_each(|i| {
        counts
            .entry(i)
            .and_modify(|c| {
                *c += 1;
            })
            .or_insert(1);
    });
    first
        .into_iter()
        .map(|i| *i * counts.get(i).copied().unwrap_or(0))
        .sum()
}

fn part2(inp: &str) -> i32 {
    let (first, second) = parse_inputs(inp);
    sim_score(&first, &second)
}

fn main() {
    println!("Part 1: {}", part1(&inputs()));
    println!("Part 2: {}", part2(&inputs()));
}
