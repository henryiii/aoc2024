pub trait Problem {
    fn solution_a(&self, input: &str) -> i64;
    fn solution_b(&self, input: &str) -> i64;
}

fn read_input(file: &str) -> String {
    std::fs::read_to_string(file).unwrap()
}

pub fn run(name: &str, problem: &dyn Problem) {
    let input = read_input(&format!("data/{name}.txt"));
    println!("Solution A: {}", problem.solution_a(&input));
    println!("Solution B: {}", problem.solution_b(&input));
}
