use std::time::Instant;

use counting_sort::counting_sort;
use rand::{distributions::Uniform, Rng};

pub mod counting_sort;

fn main() {

    let number_of_elements = 1000000;

    println!("We generate a list of {} random elements .We sort these using our own implementation of counting sort and the standard sorting algorithm in Rust. We compare how long it takes for these to sort our list.\n", number_of_elements);

    let range = Uniform::from(0..20);
    let mut values: Vec<i8> = rand::thread_rng().sample_iter(&range).take(1000000).collect();

    let values_clone = values.clone();

    let now = Instant::now();

    counting_sort(values_clone);

    let elapsed = now.elapsed();

    println!("counting sort: {:?}", elapsed);

    let now_2 = Instant::now();

    values.sort();

    let elapsed_2 = now_2.elapsed();

    println!("rust sort: {:?}", elapsed_2);
}
