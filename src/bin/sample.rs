use aoc2024::{run, Problem};

pub struct ExampleDay {}

impl Problem for ExampleDay {
    fn solution_a(&self, input: &str) -> i64 {
        input.lines().map(|line| line.parse::<i64>().unwrap()).sum()
    }

    fn solution_b(&self, input: &str) -> i64 {
        input
            .lines()
            .map(|line| line.parse::<i64>().unwrap())
            .map(|x| x * x)
            .sum()
    }
}

fn main() {
    let problem = ExampleDay {};
    run("sample", &problem);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_day() {
        let input = "1\n2\n3\n";

        let problem = ExampleDay {};
        assert_eq!(problem.solution_a(input), 6);
        assert_eq!(problem.solution_b(input), 14);
    }
}
