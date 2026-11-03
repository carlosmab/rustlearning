use std::thread;
use rand::seq::SliceRandom;

pub fn is_prime(number: u64) -> bool {
    if number < 2 {
        return false;
    }

    let mut div = 2;

    while div * div <= number {
        if number % div == 0 {
            return false;
        }
        div += 1;
    } 

    true
}

pub fn count_primes(limit: u64) -> usize {
    // let mut current = 2;
    // let mut count = 0; 

    // while current <= limit {
    //     if is_prime(current) {
    //         count += 1;
    //     }
    //     current += 1;
    // }

    // count
    (0..=limit).filter(|&n| is_prime(n)).count()
}


pub fn count_primes_parallel(limit: u64, threads: u64) -> usize {
    let chunk = limit / threads + 1;

    thread::scope(|s| {
        let handles: Vec<_> = (0..threads)
            .map(|t| {
                let start = t * chunk;
                let end = ((t + 1) * chunk).min(limit + 1);
                s.spawn(move || (start..end).filter(|&n| is_prime(n)).count())
            })
            .collect();
        
        handles.into_iter().map(|h| h.join().unwrap()).sum() 
    })
}

pub fn count_primes_shuffled(limit: u64, threads: u64) -> usize {
    let t0 = std::time::Instant::now();
    let mut numbers: Vec<u64> = (0..=limit).collect();
    numbers.shuffle(&mut rand::rng());
    println!("  Setup: {:?}", t0.elapsed());

    let chunk_size = numbers.len().div_ceil(threads as usize);

    thread::scope(|s| {
        let handles: Vec<_> = numbers
            .chunks(chunk_size)
            .map(|chunk| s.spawn(move || chunk.iter().copied().filter(|&n| is_prime(n)).count()))
            .collect();
        
        handles.into_iter().map(|h| h.join().unwrap()).sum()
    })

}