// use std::num

pub fn is_prime(n: usize) -> bool { // implement more efficient method using sieve of Eratosthenes later
    
    if n == 2 || n == 3 {return true}
    if n <= 1 || n % 2 == 0 || n % 3 == 0 {return false}



    let upper_bound = ((n as f64).sqrt() + 1.0) as usize / 6;
    
    for i in 1..(upper_bound+1) {
        if n % (6*i-1) == 0 {return false}
        if n % (6*i+1) == 0 {return false}
    }
    true
}

pub fn sieve_of_eratosthenes(n: usize) -> Vec<usize> {
    let mut is_prime = vec![true; n];
    is_prime[0] = false;
    is_prime[1] = false;

    for i in 2..((n as f64).sqrt() as usize + 1) {
        if is_prime[i] {
            for j in (i * i..n).step_by(i) {
                is_prime[j] = false;
            }
        }
    }

    is_prime
        .into_iter()
        .enumerate()
        .filter(|&(_, prime)| prime)
        .map(|(i, _)| i)
        .collect()
}

pub fn digit_sum(n: usize) -> usize {
    n.to_string().chars().map(|c| c.to_digit(10).unwrap() as usize).sum::<usize>()
}