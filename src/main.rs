use std::cmp;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time;
use three_mul;
use work_queue::Queue;

#[macro_use]
extern crate prettytable;
use prettytable::Table;

mod data_sets;

struct Task(Box<dyn Fn() + Send>);

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
    let cores = cmp::max(2, thread::available_parallelism().unwrap().get() / 2);
    let queue: Queue<Task> = Queue::new(cores, 64);

    let rows = Arc::new(Mutex::new(Vec::new()));
    let data_sets = data_sets::get_data_sets();
    for (name, data_set) in data_sets.clone() {
        let rows = rows.clone();
        queue.push(Task(Box::new(move || {
            let (result, duration) = run_quadratic_algorithm(&data_set);
            rows.lock()
                .unwrap()
                .push(row![name, data_set.len(), "quadratic", result, duration]);
        })));
    }

    for (name, data_set) in data_sets {
        let rows = rows.clone();
        queue.push(Task(Box::new(move || {
            let (result, duration) = run_subquadratic_algorithm(&data_set);
            rows.lock().unwrap().push(row![
                name,
                data_set.len(),
                "sub-quadratic",
                result,
                duration
            ]);
        })));
    }

    let handles: Vec<_> = queue
        .local_queues()
        .map(|mut local_queue| {
            std::thread::spawn(move || {
                while let Some(task) = local_queue.pop() {
                    task.0();
                }
            })
        })
        .collect();
    for handle in handles {
        handle.join().unwrap();
    }

    let mut table = Table::new();
    table.add_row(row![
        "Data set",
        "Elements",
        "Algorithm",
        "Result",
        "Execution time (ms)"
    ]);

    rows.lock().unwrap().sort_by(|a, b| {
        a.get_cell(0)
            .unwrap()
            .get_content()
            .cmp(&b.get_cell(0).unwrap().get_content())
    });
    for row in rows.lock().unwrap().iter().cloned() {
        table.add_row(row);
    }
    table.printstd();
}
