use adventofcode2024::parsers::{Either, ParseResult, Parser, Parsers};

fn inputs() -> String {
    std::fs::read_to_string("./inputs/day3.txt").unwrap()
}

fn mul() -> impl Parser<Target = (u64, u64)> {
    Parsers::lit("mul(")
        .then(Parsers::num())
        .followed_by(",")
        .and(Parsers::num().followed_by(")"))
}

fn part1(inp: &str) -> u64 {
    match mul().apply(inp) {
        ParseResult::Success {
            value: (a, b),
            rest,
        } => a * b + (if rest.is_empty() { 0 } else { part1(rest) }),
        ParseResult::Failure => {
            if !inp.is_empty() {
                part1(&inp[1..])
            } else {
                0
            }
        }
    }
}

#[derive(Clone, Copy)]
enum ParseState {
    Enabled,
    Disabled,
}

fn part2(inp: &str, state: ParseState) -> u64 {
    if inp.is_empty() {
        return 0;
    }
    match state {
        ParseState::Disabled => match Parsers::lit("do()").apply(inp) {
            ParseResult::Failure => part2(&inp[1..], state),
            ParseResult::Success { rest, .. } => part2(rest, ParseState::Enabled),
        },
        ParseState::Enabled => match Parsers::lit("don't()").or(mul()).apply(inp) {
            ParseResult::Failure => part2(&inp[1..], state),
            ParseResult::Success { value, rest } => match value {
                Either::Left(..) => part2(rest, ParseState::Disabled),
                Either::Right((a, b)) => a * b + part2(rest, state),
            },
        },
    }
}

fn main() {
    println!("Part 1: {}", part1(&inputs()));
    println!("Part 2: {}", part2(&inputs(), ParseState::Enabled));
}
