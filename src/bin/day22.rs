use std::collections::{HashMap, HashSet};

use rayon::iter::{IntoParallelIterator, ParallelIterator};

fn mix(secret: i64, other: i64) -> i64 {
    secret ^ other
}

fn prune(secret: i64) -> i64 {
    secret % 16777216
}

fn step(mut secret: i64) -> i64 {
    secret = prune(mix(secret, secret * 64));

    secret = prune(mix(secret, secret / 32));

    prune(mix(secret, secret * 2048))
}

fn step_n(mut secret: i64, n: u64) -> i64 {
    for _ in 0..n {
        secret = step(secret);
    }
    secret
}

fn gen_n(secret: i64, n: u64) -> Vec<i64> {
    let mut output = vec![secret];
    for _ in 0..n {
        output.push(step(*output.last().unwrap()));
    }
    output
}

fn compute_seq_and_score(secrets: &[i64]) -> ([i64; 4], i64) {
    let score = secrets[4] % 10;
    let seq = [
        secrets[1] % 10 - secrets[0] % 10,
        secrets[2] % 10 - secrets[1] % 10,
        secrets[3] % 10 - secrets[2] % 10,
        secrets[4] % 10 - secrets[3] % 10,
    ];
    (seq, score)
}

fn compute_seqs(secret_seq: &[i64]) -> HashMap<[i64; 4], i64> {
    let mut output = HashMap::new();

    for window in secret_seq.windows(5) {
        let (seq, score) = compute_seq_and_score(window);
        output.entry(seq).or_insert(score);
    }

    output
}

fn parse_inputs(inp: &str) -> Vec<i64> {
    inp.lines().map(|it| it.parse().unwrap()).collect()
}

fn input() -> String {
    std::fs::read_to_string("./inputs/day22.txt").unwrap()
}

fn part1(inp: &str) -> i64 {
    parse_inputs(inp)
        .into_iter()
        .map(|it| step_n(it, 2000))
        .sum()
}

fn part2(inp: &str) -> i64 {
    let monkeys: Vec<_> = parse_inputs(inp)
        .into_iter()
        .map(|it| gen_n(it, 2000))
        .map(|it| compute_seqs(&it))
        .collect();
    let all_seqs: HashSet<_> = monkeys.iter().flat_map(|it| it.keys().cloned()).collect();
    all_seqs
        .into_par_iter()
        .map(|k| {
            monkeys
                .iter()
                .map(|m| m.get(&k).copied().unwrap_or(0))
                .sum()
        })
        .max()
        .unwrap()
}

fn main() {
    println!("Part 1: {}", part1(&input()));
    println!("Part 2: {}", part2(&input()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p2() {
        let inp = "1\n2\n3\n2024";
        assert_eq!(part2(inp), 23);
    }
}
