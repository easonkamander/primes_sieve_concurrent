fn coprime<'a>(iter: impl Iterator<Item = &'a u64>, item: u64) -> bool {
    iter.copied()
        .take_while(|prime| prime * prime <= item)
        .all(|prime| item % prime != 0)
}

pub struct Sieve {
    /// An initial segment of the prime numbers
    primes: Vec<u64>,
    /// Positive integers less than the product of self.primes and coprime with self.primes
    sieved: Vec<u64>,
}

impl Default for Sieve {
    fn default() -> Self {
        Self {
            primes: vec![2, 3],
            sieved: vec![1, 5],
        }
    }
}

impl Sieve {
    pub fn run(mut self) -> Self {
        let prime = self.sieved[1];
        let bound = self.primes.iter().fold(1, |a, b| a * b);
        self.primes.push(prime);
        Self {
            primes: self.primes,
            sieved: Vec::from_iter((0..prime).flat_map(|epoch| {
                self.sieved
                    .iter()
                    .copied()
                    .map(move |n| epoch * bound + n)
                    .filter(|n| n % prime != 0)
            })),
        }
    }

    fn seq_get(&mut self) -> usize {
        let prod = self.primes.iter().fold(1, |a, b| a * b);
        let sqrt = (prod as f64).sqrt() as u64;
        let init = self.primes.len();

        for (indx, &item) in self.sieved.iter().enumerate().skip(1) {
            if item > sqrt {
                return indx;
            } else if coprime(self.primes[init..].iter(), item) {
                self.primes.push(item);
            }
        }

        self.sieved.len()
    }

    pub fn get(mut self) -> Vec<u64> {
        let start = self.seq_get();
        let sqrts = std::sync::Arc::new(self.primes);

        use core::array::from_fn;
        const WORKERS: usize = 4;

        let section: [_; WORKERS] = from_fn(|i| {
            let delta = self.sieved.len() - start;
            let lhs = start + delta * i / WORKERS;
            let rhs = start + delta * (i + 1) / WORKERS;
            Vec::from(&self.sieved[lhs..rhs])
        });
        let handles = section.map(|sect| {
            let sqrts = sqrts.clone();
            std::thread::spawn(move || {
                Vec::from_iter(sect.into_iter().filter(|&item| coprime(sqrts.iter(), item)))
            })
        });
        let results = handles.map(|handle| handle.join().unwrap());

        let mut primes = std::sync::Arc::into_inner(sqrts).unwrap();
        for result in results {
            primes.extend(result);
        }
        primes
    }
}

impl Sieve {
    pub(super) fn size(&self) -> usize {
        self.sieved.len()
    }

    pub(super) fn last(&self) -> u64 {
        self.sieved.last().unwrap().clone()
    }
}
