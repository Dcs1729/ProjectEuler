use crate::utilities::{sieve_of_eratosthenes};
use std::fmt;

struct RevPrime {
    p: usize,
}

impl fmt::Debug for RevPrime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Customize the display format here
        write!(f, "({}, {})", self.p, r(self.p))
    }
}

// impl RevPrime {
//     fn new(p: usize) -> RevPrime {
//         RevPrime {p}
//     }
// }

impl RevPrime {
    fn pair_sum(&self) -> usize {
        self.p + r(self.p)
    }
}

pub fn run() {
    const N: usize = usize::pow(10, 8);
    // const SMALL_N: usize = 10;

    // let divs: Vec<usize> = sieve_of_eratosthenes(SMALL_N);
    let mut prime_squares: Vec<usize> = sieve_of_eratosthenes(N)
        .iter()
        .map(|p| p.pow(2))
        .collect();
    
    // for i in divs {
    //     // println!("{}", i);
    //     prime_squares.retain(|p| r(*p) % i != 0 || r(*p) == i.pow(2));
    // }

    let mut prime_squares_rev: Vec<usize> = prime_squares
        .iter()
        .map(|p| r(*p))
        .collect();

    prime_squares.retain(|p| *p < r(*p));
    prime_squares_rev.retain(|p| *p < r(*p));
    prime_squares_rev.sort();

    // println!("{:?}", prime_squares);
    // println!("{:?}", prime_squares_rev);
    println!("Generated Primes\nSearching For Pairs...\n");

    let mut rps = Vec::<RevPrime>::new();
    let n = prime_squares.len(); let m = prime_squares_rev.len();
    let mut i = 0; let mut j = 0;
    while i < n && j < m {
        if prime_squares[i] == prime_squares_rev[j] {
            rps.push(RevPrime {p: prime_squares[i]});
            i += 1; j += 1;
        }
        else if prime_squares[i] < prime_squares_rev[j] {
            i += 1;
        }
        else { // prime_squares[i] > prime_squares_rev[j]
            j += 1;
        }
    }
    println!("{} Pairs", rps.len());
    println!("{:#?}", rps);
    println!("Sum: {}", rps.iter().map(|rp| rp.pair_sum()).sum::<usize>());
}


pub fn run_old() {

    const BIG_N: u64 = 169 + 961 + 12769 + 96721 + 1042441 + 1442401 + 1062961 + 1692601 + 1216609 + 9066121 + 121066009 + 900660121 + 12148668841 + 14886684121 + 12367886521 + 12568876321 + 1000422044521 + 1254402240001 + 1002007006009 + 9006007002001 + 1020506060401 + 1040606050201 + 1210684296721 + 1276924860121 + 1212427816609 + 9066187242121 + 1212665666521 + 1256665662121 + 1214648656321 + 1236568464121 + 1234367662441 + 1442667634321 + 100042424498641 + 146894424240001 + 100222143232201 + 102232341222001 + 100240164024001 + 100420461042001 + 100402824854641 + 146458428204001 + 102012282612769 + 967216282210201 + 102014060240401 + 104042060410201 + 121002486012769 + 967210684200121 + 121264386264121 + 121462683462121 + 123212686214641 + 146412686212321;
    println!("{}", BIG_N);

    const N: usize = usize::pow(10, 8);
    const SMALL_N: usize = 100;

    let divs: Vec<usize> = sieve_of_eratosthenes(SMALL_N);
    let mut prime_squares: Vec<usize> = sieve_of_eratosthenes(N)
        .iter()
        .map(|p| p.pow(2))
        .collect();
    
    for i in divs {
        println!("{}", i);
        prime_squares.retain(|p| r(*p) % i != 0 || r(*p) == i.pow(2));
    }

    // if this doesn't work, can i not just take the two lists p and r(p) and go through them linearly lol
    // and just pop from whichever one is appropriate
        

    println!("{} primes: {:?}", prime_squares.len(), prime_squares.iter().rev().take(10).rev().collect::<Vec<_>>());

    let binding = prime_squares
        .clone();
    let prime_squares_x: Vec<&usize> = binding
        .iter()
        .filter(|p| **p > r(**p))
        .collect();
    
    let psc: Vec<usize> = prime_squares
        .clone();

    let rps: Vec<(usize, usize)> = psc
        .iter()
        .filter(|p| **p < r(**p))
        .map(|p| r(*p))
        .filter(|p| prime_squares_x.contains(&p))
        .map(|p| (r(p), p))
        .collect();

    println!("{} RPS pairs: {:?}", rps.len(), rps)
}

fn r(p: usize) -> usize { // reverses number
    p
        .to_string()
        .chars()
        .rev()
        .collect::<String>()
        .parse::<usize>()
        .unwrap()
}