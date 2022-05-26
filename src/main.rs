use std::thread;
use std::time;
use three_mul;

#[macro_use] extern crate prettytable;
use prettytable::Table;

mod data_sets;

fn run_quadratic_algorithm(input: &[u64]) -> (u64, u128) {
    let start = time::Instant::now();
    let result = three_mul::quadratic_algorithm(&input);
    (result, start.elapsed().as_millis())
}

fn run_subquadratic_algorithm(input: &[u64]) -> (u64, u128) {
    let start = time::Instant::now();
    let result = three_mul::subquadratic_algorithm(&input);
    (result, start.elapsed().as_millis())
}

fn main() {
    let mut table = Table::new();
    table.add_row(row!["Data set", "elements", "quad algorithm result", "sub-quad algorithm result", "quad algorithm time", "sub-quad algorithm time"]);
    for (name, data_set) in data_sets::get_data_sets() {
        let thread_data_set = data_set.clone();
        let thread = thread::spawn(move || run_quadratic_algorithm(&thread_data_set));
        let (subquadratic_result, subquadratic_time) = run_subquadratic_algorithm(&data_set);
        let (quadratic_result, quadratic_time) = thread.join().unwrap();
        table.add_row(row![name, data_set.len(), quadratic_result, subquadratic_result, quadratic_time, subquadratic_time]);
    }
    table.printstd();
}
