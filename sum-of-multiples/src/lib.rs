use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut distinct_multiples = HashSet::new();

    for f in factors {
        for m in multiples_of(*f).take_while(|&x| x < limit) {
            distinct_multiples.insert(m);
        }
    }

    distinct_multiples.iter().sum()
}

struct Multiples {
    of: u32,
    multiplier: u32,
}

impl Iterator for Multiples {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        let current_multiplier = self.multiplier;
        self.multiplier += 1;
        println!("Of: {}, Current mul: {}", self.of, current_multiplier);

        match self.of {
            0 => None,
            _ => Some(self.of * current_multiplier),
        }
    }
}
fn multiples_of(n: u32) -> Multiples {
    Multiples {
        of: n,
        multiplier: 1,
    }
}
