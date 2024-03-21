use std::{iter, fmt};
use crate::utilities::{sieve_of_eratosthenes, is_prime};
use itertools::Itertools;

pub fn run() {
    const N: usize = 20000;
    let prime_list = sieve_of_eratosthenes(N);

    println!("no of primes: {}", prime_list.len());

    let x = prime_pair_sets(&prime_list, 5);
    println!("{:?}", x);
}

#[derive(Clone)]
struct PrimeIndices {
    indices: Vec<usize>,
}

impl PrimeIndices {
    fn new(indices: Vec<usize>) -> Self {
        PrimeIndices { indices }
    }

    fn top(&self) -> Option<usize> {
        self.indices.last().copied()
    }

    fn push(&mut self, x: usize) {
        self.indices.push(x);
    }

    fn clone_append(&self, x: usize) -> Self {
        let mut new_pi = self.clone();
        new_pi.push(x);
        new_pi
    }
    fn valid_add(&self, prime_list: &[usize], new_i: usize) -> bool {
        self.indices
            .iter()
            .map(|i| {
                let p1 = concat(prime_list[*i], prime_list[new_i]);
                let p2 = concat(prime_list[new_i], prime_list[*i]);

                (is_prime(p1), is_prime(p2))
            })
            .all(|(a, b)| a && b) // both concats are primes for all combinations
    }

    fn prime_vec(&self, prime_list: &[usize]) -> Vec<usize> {
        self.indices
            .iter()
            .map(|i| prime_list[*i])
            .collect::<Vec<usize>>()
    }
}

impl fmt::Debug for PrimeIndices {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.indices)
    }
}

fn concat(a: usize, b: usize) -> usize {
    let concat_n = a.to_string() + &b.to_string();
    concat_n.parse::<usize>().unwrap()
}

fn prime_pair_sets(prime_list: &[usize], set_size: usize) -> Vec<Vec<usize>> { // we need this to convert the final form of sets from PrimeIndices to Vec<usize>
    prime_pair_sets_rec(&prime_list, set_size)
        .iter()
        .map(|p_set| p_set.prime_vec(&prime_list))
        .collect()
}

fn prime_pair_sets_rec(prime_list: &[usize], set_size: usize) -> Vec<PrimeIndices> { // recursive function for prime_pair_sets to do the actual legwork
    if set_size <= 1 {
        (0..prime_list.len()).map(|p| PrimeIndices::new(vec![p])).collect::<Vec<_>>()
    }
    else {
        let x = prime_pair_sets_rec(&prime_list, set_size - 1)
            .iter()
            .map(|set| {
                (set.top().unwrap()..prime_list.len())
                    .into_iter()
                    .filter(|i| set.valid_add(prime_list, *i))
                    .map(|i| set.clone_append(i))
                    .collect::<Vec<_>>()
            })
            .flatten()
            .collect::<Vec<_>>();

        println!("found {} sets of size {}:", x.len(), set_size);
        x
    }
}