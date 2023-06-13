// threads1.rs
// Execute `rustlings hint threads1` or use the `hint` watch subcommand for a hint.

// This program spawns multiple threads that each run for at least 250ms,
// and each thread returns how much time they took to complete.
// The program should wait until all the spawned threads have finished and
// should collect their return values into a vector.

use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

const THREAD_COUNT: u64 = 10;

fn main() {
    let start = Instant::now();

    let handles: Vec<JoinHandle<u128>> = (0..THREAD_COUNT)
        .map(|i| {
            thread::spawn(move || {
                let start = Instant::now();
                thread::sleep(Duration::from_millis(250 * i));
                println!("thread {} is complete", i);
                start.elapsed().as_millis()
            })
        })
        .collect();

    let results: Vec<u128> = handles
        .into_iter()
        .map(|handle| handle.join().unwrap())
        .collect();

    if results.len() != (THREAD_COUNT as usize) {
        panic!("Oh no! All the spawned threads did not finish!");
    }

    println!();
    for (i, result) in results.into_iter().enumerate() {
        println!("thread {} took {}ms", i, result);
    }

    println!("\nmain took {}ms to run.", start.elapsed().as_millis());
}
