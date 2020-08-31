use core::cmp::max;
use std::time::SystemTime;

fn main() {
    const N: u64 = 500_000;
    let now = SystemTime::now();
    let primes = calc_primes(N);

    match now.elapsed() {
        Ok(elapsed) => {
            println!("Time: {}ms", elapsed.as_millis());
        }
        Err(_) => {}
    }

    println!("The {}th prime number is {}", N, primes.iter().last().map_or(0, |&v| v));
}

// Returns the first n primes
fn calc_primes(n: u64) -> Vec<u64> {
    let mut pre_sieved = [0, 0, 2].to_vec();

    let mut upper_bound = 2;
    loop {
        let mut primes = sieve_up_to(upper_bound, &mut pre_sieved);
        upper_bound *= 2;
        if primes.len() >= n as usize {
            primes.truncate(n as usize);
            return primes;
        }
    }
}

// Returns all primes up to a given boundary n using the sieve of Eratosthenes
// The underlying sieve is returned and can be reused for future iterations using a greater boundary
// sieve[i] == 0 -> i is not a prime
// sieve[i] == i -> i is prime
fn sieve_up_to(n: u64, sieve: &mut Vec<u64>) -> Vec<u64> {
    // Init minimal sieve
    if sieve.len() < 3 {
        sieve.clear();
        sieve.append(&mut vec![0, 0, 2]);
    }

    let previous_length = sieve.len() as u64;
    let last_prime: u64 = match sieve.iter().rev().find(|&&x| x != 0) {
        Some(p) => *p,
        None => 2,
    };

    // Given sieve is expanded such that sieve[i] == i --> expanded area still needs to be tested
    let mut val_to_add = sieve.len() as u64 - 1;
    sieve.resize_with(n as usize, || {
        val_to_add += 1;
        val_to_add
    });

    // Mark non-primes from already known primes in expanded area
    for prime_index in 2..last_prime {
        if sieve[prime_index as usize] != 0 {
            // Start with a factor, such that prime*factor starts closer to the beginning of the expanded area
            // Example: * previous sieve = [0,0,2,3,0,5]
            //          * current sieve = [0,0,2,3,0,5,6,7,8,9,10,11]
            //          * prime_index = 2
            //          * factor starts at 3 so we skip marking 4 as non-prime (we already know it isn't)
            //let mut factor = max(prime_index, last_prime / prime_index);
            let mut factor = max(prime_index, previous_length / prime_index);
            while prime_index * factor < sieve.len() as u64 {
                sieve[(prime_index * factor) as usize] = 0;
                factor += 1;
            }
        }
    }

    // Continue to mark non-primes from those that have been newly found
    for prime_index in last_prime..sieve.len() as u64 {
        if sieve[prime_index as usize] != 0 {
            // Start with the factor as the square of the current prime number (default algorithm for Eratosthenes)
            let mut factor = max(2, prime_index);
            while prime_index * factor < sieve.len() as u64 {
                sieve[(prime_index * factor) as usize] = 0;
                factor += 1;
            }
        }
    }

    let primes: Vec<u64> = sieve.iter().filter(|&&x| x != 0).map(|x| *x).collect();

    primes
}
