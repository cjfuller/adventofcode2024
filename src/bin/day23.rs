use std::collections::HashSet;

fn input() -> String {
    std::fs::read_to_string("./inputs/day23.txt").unwrap()
}

struct NodeTriad<'a>(&'a str, &'a str, &'a str);

impl PartialEq for NodeTriad<'_> {
    fn eq(&self, other: &Self) -> bool {
        let mut self_elts = [self.0, self.1, self.2];
        let mut other_elts = [other.0, other.1, other.2];
        self_elts.sort();
        other_elts.sort();
        self_elts == other_elts
    }
}

impl Eq for NodeTriad<'_> {}

impl std::hash::Hash for NodeTriad<'_> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let mut self_elts = [self.0, self.1, self.2];
        self_elts.sort();
        self_elts.hash(state)
    }
}

fn parse_input(inp: &str) -> HashSet<(&str, &str)> {
    let mut output = HashSet::new();
    inp.lines().for_each(|l| {
        let parts = l.split("-").collect::<Vec<_>>();
        output.insert((parts[0], parts[1]));
        output.insert((parts[1], parts[0]));
    });

    output
}

fn part1(inp: &str) -> usize {
    let graph = parse_input(inp);
    let mut triads: HashSet<NodeTriad> = HashSet::new();

    graph.iter().for_each(|(k0, k1)| {
        if k0.starts_with('t') {
            graph
                .iter()
                .filter(|(k2, k3)| k2 == k0 && k3 != k1)
                .for_each(|(_, k3)| {
                    graph
                        .iter()
                        .filter(|(k4, k5)| k4 == k1 && k5 != k0)
                        .for_each(|(_, k5)| {
                            if k3 == k5 {
                                triads.insert(NodeTriad(k0, k1, k3));
                            }
                        })
                })
        }
    });
    triads.len()
}

fn main() {
    println!("Part 1: {}", part1(&input()));
    todo!()
}
