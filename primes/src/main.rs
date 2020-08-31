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

    println!("{}", primes.iter().last().map_or(0, |&v| v));
}

// Returns the first n primes
fn calc_primes(n: u64) -> Vec<u64> {
    let mut pre_sieved = [0, 0, 2].to_vec();

    let mut upper_bound = 2;
    loop {
        //println!("Calc primes until upper bound {}", upper_bound);
        let mut primes = sieve_up_to(upper_bound, &mut pre_sieved);
        //println!("Primes for upper bound {} : {:?}", upper_bound, primes);
        upper_bound *= 2;
        if primes.len() >= n as usize {
            primes.truncate(n as usize);
            return primes;
        }
    }
}

// Returns all primes up to a given boundary n using the sieve of Eratosthenes
// The underlying sieve is returned and can be reused for future iterations using a greater boundary
fn sieve_up_to(n: u64, sieve: &mut Vec<u64>) -> Vec<u64> {
    if sieve.len() < 3 {
        sieve.clear();
        sieve.append(&mut vec![0, 0, 2]);
    }

    let last_prime: u64 = match sieve.iter().rev().find(|&&x| x != 0) {
        Some(p) => *p,
        None => 2,
    };

    //println!("Last prime: {}", last_prime);

    let mut val_to_add = sieve.len() as u64 - 1;
    sieve.resize_with(n as usize, || {
        val_to_add += 1;
        val_to_add
    });

    //println!("Mark non-primes from already known primes in resized area");
    // Mark non-primes from already known primes in resized area
    for prime_index in 2..last_prime {
        //println!("Prime Index: {}",prime_index);
        let mut factor = max(prime_index, last_prime / prime_index); // XXX
        if sieve[prime_index as usize] != 0 {
            while prime_index * factor < sieve.len() as u64 {
                /*
                println!(
                    "Index {} * Factor {} = {}",
                    prime_index,
                    factor,
                    prime_index * factor
                );
                */
                sieve[(prime_index * factor) as usize] = 0;
                factor += 1;
            }
        }
    }

    // println!("Mark remaining non-primes");
    for prime_index in last_prime..sieve.len() as u64 {
        //   println!("Prime Index: {}",prime_index);
        let mut factor = max(2, prime_index);
        if sieve[prime_index as usize] != 0 {
            while prime_index * factor < sieve.len() as u64 {
                /*
                println!(
                    "Index {} * Factor {} = {}",
                    prime_index,
                    factor,
                    prime_index * factor
                );*/
                sieve[(prime_index * factor) as usize] = 0;

                factor += 1;
            }
        }
    }

    let primes: Vec<u64> = sieve.iter().filter(|&&x| x != 0).map(|x| *x).collect();
    /*
       let mut primes = Vec::new();
        for number in sieve {
            if *number != 0 {
                primes.push(*number);
            }
        }
    */
    primes
}
