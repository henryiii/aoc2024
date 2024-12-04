pub trait Problem {
    fn solution_a(input: &str) -> i64;
    fn solution_b(input: &str) -> i64;
}

fn read_input(file: &str) -> String {
    std::fs::read_to_string(file).unwrap()
}

pub fn run<T: Problem>(name: &str) {
    let input = read_input(&format!("data/{name}.txt"));
    println!("Solution A: {}", T::solution_a(&input));
    println!("Solution B: {}", T::solution_b(&input));
}
