use std::iter;
use std::ops::RangeFrom;

fn main() {
    let p: Vec<usize> = primes(10);

    println!("{:?}", p);
    assert_eq!(4, p.len());

    let p: Vec<u8> = primes(30);

    println!("{:?}", p);
    assert_eq!(10, p.len());

    let p: Vec<u64> = primes(100);

    println!("{:?}", p);
    assert_eq!(25, p.len());
}

fn primes<T>(n: usize) -> Vec<T> where
   T: From<u8>,
   RangeFrom<T>: Iterator<Item=T> {
    let mut sieve: Vec<bool> = iter::repeat(true).take(n).collect();
    
    let limit = (sieve.len() as f32).sqrt().ceil() as usize;
    for i in 2..limit {
        if sieve[i] {
            // it would be nice to use step_by here, but it isnt stable
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
    
    (T::from(0u8)..).zip(sieve.into_iter())
         .filter(|&(_, is_prime)| is_prime)
         .map(|(idx, _)| idx)
         .skip(2)
         .collect()
}
