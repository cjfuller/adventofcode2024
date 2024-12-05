use std::fmt::Debug;
use std::marker::PhantomData;

use regex::Regex;

#[derive(Eq, PartialEq)]
pub enum ParseResult<'a, T: Eq> {
    Success { value: T, rest: &'a str },
    Failure,
}

impl<T: Debug + Eq> Debug for ParseResult<'_, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Success { value, rest } => f
                .debug_struct("Success")
                .field("value", value)
                .field("rest", rest)
                .finish(),
            Self::Failure => write!(f, "Failure"),
        }
    }
}

pub trait Parser: Sized {
    type Target: Eq;
    fn apply<'a>(&self, target: &'a str) -> ParseResult<'a, Self::Target>;

    fn and<P: Parser>(self, other: P) -> impl Parser<Target = (Self::Target, P::Target)> {
        ParserSeq {
            first: self,
            second: other,
        }
    }

    fn then<P: Parser>(self, other: P) -> impl Parser<Target = P::Target> {
        self.and(other).map(|(_, r)| r)
    }

    fn followed_by<P: Parser>(self, other: P) -> impl Parser<Target = Self::Target> {
        self.and(other).map(|(l, _)| l)
    }

    fn or<P: Parser>(self, other: P) -> impl Parser<Target = Either<Self::Target, P::Target>> {
        OrParser {
            first: self,
            second: other,
        }
    }

    fn map<U: Eq, F: Fn(Self::Target) -> U>(self, mapper: F) -> impl Parser<Target = U> {
        MappedParser {
            orig: self,
            mapper,
            _u: PhantomData,
        }
    }
}

pub struct Parsers;
impl Parsers {
    pub fn r(re: &str) -> impl Parser<Target = String> {
        RegexParser::new(re)
    }
    pub fn lit<S: AsRef<str>>(s: S) -> Lit<S> {
        Lit { s }
    }
    pub fn num() -> impl Parser<Target = u64> {
        Self::r("[0-9]+").map(|it| it.parse().unwrap())
    }
}

pub struct MappedParser<U, P: Parser, F: Fn(P::Target) -> U> {
    orig: P,
    mapper: F,
    // If I don't include the `U` parameter, the compiler wants F::Output to be () unfortunately. I
    // think I'd need the unstable alternate syntax for the Fn trait to write it correctly?
    _u: PhantomData<U>,
}

impl<U: Eq, P: Parser, F: Fn(P::Target) -> U> Parser for MappedParser<U, P, F> {
    type Target = F::Output;

    fn apply<'a>(&self, target: &'a str) -> ParseResult<'a, Self::Target> {
        match self.orig.apply(target) {
            ParseResult::Failure => ParseResult::Failure,
            ParseResult::Success { value, rest } => ParseResult::Success {
                value: (self.mapper)(value),
                rest,
            },
        }
    }
}

pub struct ParserSeq<P: Parser, Q: Parser> {
    first: P,
    second: Q,
}

impl<P: Parser, Q: Parser> Parser for ParserSeq<P, Q> {
    type Target = (P::Target, Q::Target);

    fn apply<'a>(&self, target: &'a str) -> ParseResult<'a, Self::Target> {
        match self.first.apply(target) {
            ParseResult::Failure => ParseResult::Failure,
            ParseResult::Success { value, rest } => match self.second.apply(rest) {
                ParseResult::Failure => ParseResult::Failure,
                ParseResult::Success {
                    value: value2,
                    rest: rest2,
                } => ParseResult::Success {
                    value: (value, value2),
                    rest: rest2,
                },
            },
        }
    }
}

#[derive(Eq, PartialEq)]
pub enum Either<L: Eq, R: Eq> {
    Left(L),
    Right(R),
}

#[derive(Eq, PartialEq)]
pub struct OrParser<P: Parser, Q: Parser> {
    first: P,
    second: Q,
}

impl<P: Parser, Q: Parser> Parser for OrParser<P, Q> {
    type Target = Either<P::Target, Q::Target>;

    fn apply<'a>(&self, target: &'a str) -> ParseResult<'a, Self::Target> {
        match self.first.apply(target) {
            ParseResult::Success { value, rest } => ParseResult::Success {
                value: Either::Left(value),
                rest,
            },
            ParseResult::Failure => match self.second.apply(target) {
                ParseResult::Success { value, rest } => ParseResult::Success {
                    value: Either::Right(value),
                    rest,
                },
                ParseResult::Failure => ParseResult::Failure,
            },
        }
    }
}

pub struct RegexParser {
    re: Regex,
}

impl RegexParser {
    pub fn new(s: &str) -> RegexParser {
        Self {
            re: Regex::new(s).unwrap(),
        }
    }
}

impl Parser for RegexParser {
    type Target = String;

    fn apply<'a>(&self, target: &'a str) -> ParseResult<'a, Self::Target> {
        match self.re.find_at(target, 0) {
            Some(m) => ParseResult::Success {
                value: m.as_str().to_string(),
                rest: target.get(m.end()..).unwrap_or(""),
            },
            None => ParseResult::Failure,
        }
    }
}

pub struct Lit<S: AsRef<str>> {
    s: S,
}

impl<S: AsRef<str>> Parser for Lit<S> {
    type Target = String;

    fn apply<'a>(&self, target: &'a str) -> ParseResult<'a, Self::Target> {
        if target.starts_with(self.s.as_ref()) {
            ParseResult::Success {
                value: self.s.as_ref().to_string(),
                rest: target.get(self.s.as_ref().len()..).unwrap_or(""),
            }
        } else {
            ParseResult::Failure
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parsing_num() {
        let parser = Parsers::num();
        let res = parser.apply("104");
        assert_eq!(
            res,
            ParseResult::Success {
                value: 104,
                rest: ""
            }
        );
    }
    #[test]
    fn test_parsing_lit() {
        let parser = Parsers::lit("abcd");
        let res = parser.apply("abcdefg");
        assert_eq!(
            res,
            ParseResult::Success {
                value: "abcd".into(),
                rest: "efg"
            }
        )
    }
    #[test]
    fn test_chaining() {
        let parser = Parsers::num().and(Parsers::lit(" calling birds"));
        let res = parser.apply("4 calling birds");
        assert_eq!(
            res,
            ParseResult::Success {
                value: (4, " calling birds".into()),
                rest: ""
            }
        );
    }
    #[test]
    fn test_parsing_complex() {
        let parser = Parsers::lit("mul(")
            .then(Parsers::num())
            .followed_by(Parsers::lit(","))
            .and(Parsers::num().followed_by(Parsers::lit(")")));
        let res = parser.apply("mul(32, 3)");
        assert_eq!(
            res,
            ParseResult::Success {
                value: (32, 3),
                rest: ""
            }
        );
    }
}
