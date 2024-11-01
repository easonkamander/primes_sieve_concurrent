mod sieve;
use sieve::Sieve;

fn main() {
    let timer = std::time::Instant::now();
    let sieve = Sieve::exec_exact(9);
    println!("Time: {}ms", timer.elapsed().as_millis());
    let mut i = 10;
    while i < sieve.len() {
        println!("The {}th prime is {}", i, sieve[i]);
        i *= 10;
    }
}
