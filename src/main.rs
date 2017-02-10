use std::iter;

fn main() {
    println!("{:?}", primes(10));
    assert_eq!(4, primes(10).len());

    println!("{:?}", primes(30));
    assert_eq!(10, primes(30).len());

    println!("{:?}", primes(100));
    assert_eq!(25, primes(100).len());
}

fn primes(n: usize) -> Vec<usize> {
    let mut sieve: Vec<bool> = iter::repeat(true).take(n).collect();
    
    let limit = (sieve.len() as f32).sqrt().ceil() as usize;
    for i in 2..limit {
        if sieve[i] {
            let mut multiple = i * i;
            loop {
                if multiple >= sieve.len() {
                    break;
                }
                sieve[multiple] = false;
                multiple = multiple + i;
            }
        }
    }
    
    sieve.iter().enumerate().filter(|&(_, is_prime)| *is_prime).map(|(idx, _)| idx).skip(2).collect()
}
