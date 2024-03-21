use std::{iter, fmt};
use crate::utilities::{sieve_of_eratosthenes, is_prime};
use itertools::Itertools;

pub fn run() {
    const SMALL_N: usize = 10000;
 
    // const BIG_N = SMALL_N.pow(2);   
    let prime_list = sieve_of_eratosthenes(SMALL_N);

    println!("no of primes: {}", prime_list.len());

    // let check_vec = vec![719, 197, 797, 977, 1997, 9719];
    // for p in check_vec {
    //     println!("{}: {}", p, is_prime(p));
    // }

    // let p_list_1: Vec<_> = p_list_mod3(&prime_list, 1); // primes equal to 1 mod 3, excl. 2 incl. 3
    // let p_list_2: Vec<_> = p_list_mod3(&prime_list, 2); // primes equal to 2 mod 3, excl. 2 incl. 3
            
    // println!("{} {}\n", p_list_1.len(), p_list_2.len());

    

    // let p_combs: Vec<_> = iter::once(3)
    //     .chain(p_list_2)
    //     .combinations(4)
    //     .collect();
    


    // println!("{:?}\n", p_combs.len());

    // let pps: Vec<_> = p_combs
    //     .iter()
    //     .filter(|set| all_prime(all_concats(*set)))
    //     .collect();
    
    // println!("{:?}", pps);

    
   
    
    // let mut y = PrimeIndices::new(vec![7,8,9]);
    // println!("{}", y.top().unwrap());
    // y.push(10);
    // println!("{}", y.top().unwrap());
    
    

    // let nums = vec![1,2,3,4,5];
    // println!("{:?}", all_concats(nums));


    // alternative idea: build up from say all sets of 3 primes with PPS property to sets of 4 with PPS, because any set of 4 must come from a set of 3
    // and repeat iteratively from start (set of 1 (i.e. all primes))

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

// fn all_concats(v: &Vec<usize>) -> Vec<usize> {
//     let mut x = v
//         .iter()
//         .combinations(2)
//         .map(|i| {let p1 = i[0]; let p2 = i[1]; (p1, p2)})
//         .map(|(p1, p2)| vec![(p1,p2), (p2,p1)])
//         .flatten()
//         .map(|(p1, p2)| concat(*p1, *p2))
//         .collect::<Vec<usize>>();

//     x.sort();
//     x

// }

fn concat(a: usize, b: usize) -> usize {
    let concat_n = a.to_string() + &b.to_string();
    concat_n.parse::<usize>().unwrap()
}

// fn all_prime(ps: Vec<usize>) -> bool {
//     ps
//         .iter()
//         .map(|p| is_prime(*p))
//         .all(|p| p)
// }

// fn p_list_mod3(p_list: &Vec<usize>, n: usize) -> Vec<usize> {
//     p_list
//         .iter()
//         .skip(2)
//         .filter(|p| *p % 3 == n)
//         .copied()
//         .collect()
// }

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
        // return vec![]
        prime_pair_sets_rec(&prime_list, set_size - 1)
            .iter()
            .map(|set| {
                // println!("{:?}", (set.top().unwrap()..));
                (set.top().unwrap()..prime_list.len())
                    .into_iter()
                    .filter(|i| 
                        {
                            // println!("{}", i);
                            set.valid_add(prime_list, *i)
                        }
                    )
                    .map(|i| set.clone_append(i))
                    .collect::<Vec<_>>()

            })
            .flatten()
            .collect::<Vec<_>>()
    }
}