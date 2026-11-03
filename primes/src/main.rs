use primes::primes::{count_primes, count_primes_parallel, count_primes_shuffled};
use std::time::Instant;


fn main() {
    let limit = 10_000_000;

    let start = Instant::now();
    let total = count_primes(limit);
    println!("1 core: {total} primes in {:?}", start.elapsed());

    let threads = std::thread::available_parallelism()
        .map(|n| n.get() as u64)
        .unwrap_or(4);

    let start = Instant::now();
    let total = count_primes_parallel(limit, threads);
    println!("{threads} cores: {total} primes in {:?}", start.elapsed());

    let start = Instant::now();
    let total = count_primes_shuffled(limit, threads);
    println!("Shuffled -> {threads} cores: {total} primes in {:?}", start.elapsed());

}
