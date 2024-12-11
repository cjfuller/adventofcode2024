use std::collections::HashMap;
use std::fmt::Display;

const INPUTS: [u64; 8] = [872027, 227, 18, 9760, 0, 4, 67716, 9245696];

trait Digits: Display {
    fn num_digits(&self) -> usize {
        self.to_string().len()
    }
}

impl Digits for u64 {}

enum OneOrTwo<T> {
    One(T),
    Two(T, T),
}

struct OneOrTwoIter<T> {
    num_yielded: usize,
    data: OneOrTwo<T>,
}

impl<T: Clone> Iterator for OneOrTwoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        use OneOrTwo::*;
        let value = match (self.num_yielded, &self.data) {
            (0, One(t)) => Some(t.clone()),
            (.., One(..)) => None,
            (0, Two(t, ..)) => Some(t.clone()),
            (1, Two(.., t)) => Some(t.clone()),
            (.., Two(..)) => None,
        };
        self.num_yielded += 1;
        value
    }
}

impl<T: Clone> IntoIterator for OneOrTwo<T> {
    type Item = T;

    type IntoIter = OneOrTwoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        OneOrTwoIter {
            data: self,
            num_yielded: 0,
        }
    }
}

fn rule(num: u64) -> OneOrTwo<u64> {
    use OneOrTwo::*;
    match num {
        0 => One(1),
        n if n.num_digits() % 2 == 0 => {
            let mut n_str = n.to_string();
            let second = n_str.split_off(n_str.len() / 2);
            Two(n_str.parse().unwrap(), second.parse().unwrap())
        }
        n => One(n * 2024),
    }
}

fn part1(inp: &[u64]) -> usize {
    let mut stones = inp.to_vec();
    for _ in 0..25 {
        stones = stones.into_iter().flat_map(rule).collect();
    }

    stones.len()
}

fn solve_by_counts(inp: &[u64], n_iters: usize) -> usize {
    let mut counts: HashMap<u64, usize> = HashMap::new();
    for item in inp {
        counts.entry(*item).and_modify(|v| *v += 1).or_insert(1);
    }
    for _ in 0..n_iters {
        let mut next_counts = HashMap::new();

        for (k, v) in counts {
            let next = rule(k);
            for next_item in next {
                next_counts
                    .entry(next_item)
                    .and_modify(|v_next| *v_next += v)
                    .or_insert(v);
            }
        }

        counts = next_counts;
    }

    counts.values().sum()
}

fn part2(inp: &[u64]) -> usize {
    solve_by_counts(inp, 75)
}

fn main() {
    println!("Part 1: {}", part1(&INPUTS));
    println!("Part 1 (alt): {}", solve_by_counts(&INPUTS, 25));
    println!("Part 2: {}", part2(&INPUTS));
}
