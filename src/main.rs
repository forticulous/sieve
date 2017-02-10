use std::iter;

fn main() {
    println!("{:?}", primes(10));
    println!("{:?}", primes(30));
    println!("{:?}", primes(100));
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
    
    sieve.iter().filter(|&x| *x).enumerate().map(|(idx, _)| idx).skip(2).collect()
}
