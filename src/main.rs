mod sieve;
use sieve::Sieve;

enum Command {
    Exact(usize),
    Count(usize),
    Until(u64),
}

impl Command {
    fn exec(self) -> Vec<u64> {
        let mut sieve = Sieve::default();
        match self {
            Command::Exact(steps) => {
                for _ in 2..steps {
                    sieve = sieve.run();
                }
            }
            Command::Count(count) => {
                while sieve.size() < count {
                    sieve = sieve.run();
                }
            }
            Command::Until(value) => {
                while sieve.last() < value {
                    sieve = sieve.run();
                }
            }
        };
        sieve.get()
    }
}

enum Error {
    ParseInt(core::num::ParseIntError),
    Unrecognized(String),
}

impl From<core::num::ParseIntError> for Error {
    fn from(value: core::num::ParseIntError) -> Self {
        Self::ParseInt(value)
    }
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ParseInt(err) => write!(f, "{:?}", err),
            Self::Unrecognized(cmd) => write!(f, "Unrecognized command: {}", cmd),
        }
    }
}

fn main() -> Result<(), Error> {
    let command = if let Some(arg) = std::env::args().nth(1) {
        match arg.split_once('=') {
            Some(("exact", num)) => Command::Exact(str::parse(num)?),
            Some(("count", num)) => Command::Count(str::parse(num)?),
            Some(("until", num)) => Command::Until(str::parse(num)?),
            Some((cmd, _)) => return Err(Error::Unrecognized(cmd.into())),
            None => return Err(Error::Unrecognized(arg.into())),
        }
    } else {
        Command::Exact(8)
    };

    let timer = std::time::Instant::now();
    let sieve = command.exec();
    println!("Time: {}ms", timer.elapsed().as_millis());
    println!("Primes: {}", sieve.len());
    println!("Largest: {}", sieve.last().unwrap());

    let mut tens = 10;
    while tens < sieve.len() {
        println!("The {}th prime is {}", tens, sieve[tens]);
        tens *= 10;
    }

    Ok(())
}
