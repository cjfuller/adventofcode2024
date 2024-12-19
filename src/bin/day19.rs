use std::collections::HashMap;

use regex::Regex;

fn pattern_input() -> String {
    std::fs::read_to_string("./inputs/day19_patterns.txt").unwrap()
}

fn design_input() -> String {
    std::fs::read_to_string("./inputs/day19_designs.txt").unwrap()
}

fn parse_patterns(inp: &str) -> Regex {
    Regex::new(&format!(
        "^({})+$",
        inp.split(", ").collect::<Vec<_>>().join("|")
    ))
    .unwrap()
}

fn part1(patterns: &str, designs: &str) -> usize {
    let re = parse_patterns(patterns);
    designs.lines().filter(|it| re.is_match(it)).count()
}

fn count_matches<'a>(
    patterns: &[&str],
    design: &'a str,
    cache: &mut HashMap<&'a str, usize>,
) -> usize {
    if design == "" {
        return 1;
    }
    if let Some(precalc) = cache.get(design) {
        return *precalc;
    }
    let mut total = 0;
    for p in patterns {
        if design.starts_with(p) {
            total += count_matches(patterns, &design[p.len()..], cache)
        }
    }
    cache.insert(design, total);
    total
}

fn part2(patterns: &str, designs: &str) -> usize {
    let patterns = patterns.split(", ").collect::<Vec<_>>();
    let mut cache = HashMap::new();
    designs
        .lines()
        .map(|l| count_matches(&patterns, l, &mut cache))
        .sum()
}

fn main() {
    println!("Part 1: {}", part1(&pattern_input(), &design_input()));
    println!("Part 2: {}", part2(&pattern_input(), &design_input()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_matches() {
        let patterns = ["r", "wr", "b", "g", "bwu", "rb", "gb", "br"];
        let design = "rrbgbr";
        let mut cache = HashMap::new();
        assert_eq!(count_matches(&patterns, design, &mut cache), 6);
        let design = "gbbr";
        let mut cache = HashMap::new();
        assert_eq!(count_matches(&patterns, design, &mut cache), 4);
    }
}
