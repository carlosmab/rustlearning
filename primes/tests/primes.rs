use primes::primes::{is_prime, count_primes, count_primes_parallel, count_primes_shuffled};

#[test]
fn array_is_prime() {
    for n in [2, 3, 5, 7, 11, 13, 97] {
        assert!(is_prime(n), "{n} should be prime");
    }
}

#[test]
fn array_is_not_prime() {
    for n in [0, 1, 4, 15, 49, 100] {
        assert!(!is_prime(n), "{n} shouldn't be prime");
    }
}

#[test]
fn counts_primes_up_100() {
    assert_eq!(count_primes(100), 25);
}

#[test]
fn parallel_matches_sequential() {
    for threads in [1, 2, 4, 8] {
        assert_eq!(count_primes_parallel(10_000, threads), count_primes(10_000))
    }
}

#[test]
fn suffled_matches_sequential() {
    for threads in [1, 2, 4, 8] {
        assert_eq!(count_primes_shuffled(10_000, threads), count_primes(10_000))
    }
}