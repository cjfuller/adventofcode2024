fn inputs() -> String {
    std::fs::read_to_string("./inputs/day2.txt").unwrap()
}

enum Direction {
    Increasing,
    Decreasing,
}

struct Report {
    pub nums: Vec<i32>,
}

struct RemovalIter<'a> {
    src: &'a Report,
    next_idx: usize,
}

impl<'a> Iterator for RemovalIter<'a> {
    type Item = Report;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next_idx < self.src.nums.len() {
            let mut next_vec = self.src.nums.clone();
            next_vec.remove(self.next_idx);
            let next_val = Report { nums: next_vec };
            self.next_idx += 1;
            Some(next_val)
        } else {
            None
        }
    }
}

impl<'a> IntoIterator for &'a Report {
    type Item = Report;

    type IntoIter = RemovalIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        RemovalIter {
            src: self,
            next_idx: 0,
        }
    }
}

impl Report {
    fn is_safe(&self) -> bool {
        if self.nums.len() < 2 {
            return false;
        }
        let direction = if self.nums.first() < self.nums.last() {
            Direction::Increasing
        } else if self.nums.first() > self.nums.last() {
            Direction::Decreasing
        } else {
            return false;
        };

        for pair in self.nums.windows(2) {
            match direction {
                Direction::Decreasing => {
                    if pair[0] <= pair[1] {
                        return false;
                    } else if pair[0] - pair[1] > 3 {
                        return false;
                    }
                }
                Direction::Increasing => {
                    if pair[0] >= pair[1] {
                        return false;
                    } else if pair[1] - pair[0] > 3 {
                        return false;
                    }
                }
            }
        }
        true
    }

    fn is_safe_with_problem_dampener(&self) -> bool {
        self.is_safe() || self.into_iter().any(|it| it.is_safe())
    }
}

fn parse_inputs(inp: &str) -> Vec<Report> {
    inp.lines()
        .map(|l| Report {
            nums: l
                .split(" ")
                .filter(|it| !it.is_empty())
                .map(|it| it.parse::<i32>().unwrap())
                .collect(),
        })
        .collect()
}

fn part1(inp: &str) -> usize {
    parse_inputs(inp)
        .into_iter()
        .filter(|it| it.is_safe())
        .count()
}

fn part2(inp: &str) -> usize {
    parse_inputs(inp)
        .into_iter()
        .filter(|it| it.is_safe_with_problem_dampener())
        .count()
}

fn main() {
    println!("Part 1: {}", part1(&inputs()));
    println!("Part 2: {}", part2(&inputs()));
}
