use crate::utilities::{sieve_of_eratosthenes, is_prime};

pub fn run() {
    let squbes = gen_squbes(10_usize.pow(12));
    // let squbes = gen_squbes(229161792008+1);
    // println!("{:?}", x);
    // println!("{}\n", x.len());
    let filt_squbes = filter_200(squbes);
    // println!("{:?}", filt_squbes);
    // println!("{}", filt_squbes.len());

    let filt2_squbes = filter_pproof(filt_squbes);
    // println!("{:?}", filt2_squbes);
    println!("{}", filt2_squbes.len());

    println!("{}", filt2_squbes[200-1]);
}

fn filter_200(squbes: Vec<usize>) -> Vec<usize> {
    let filt_squbes : Vec<usize> = squbes
        .into_iter()
        .filter(|sqbe| sqbe.to_string().contains("200"))
        .collect();

    filt_squbes
}

fn filter_pproof(squbes: Vec<usize>) -> Vec<usize> {
    let filt_squbes : Vec<usize> = squbes
        .into_iter()
        .filter(|sqbe| is_pproof(*sqbe))
        .collect();

    filt_squbes
}

fn sqube(x: usize, y: usize) -> usize {
    x.pow(2) * y.pow(3)
}

fn gen_squbes(max_val: usize) -> Vec<usize> {
    let max_p = (max_val as f64 / 8.0).sqrt() as usize + 1;
    let primes = sieve_of_eratosthenes(max_p); // max_p excluded
    let p_len = primes.len();
    let mut squbes : Vec<usize> = Vec::new();

    if primes.len() <= 2 {return vec![]}

    let mut i = 0; let mut j = 1;
    loop {
        if j >= p_len {
            i += 1;
            j = 0;
        }
        if i >= p_len {break;}

        if i == j {
            j += 1;
            continue;
        }

        let sqbe = sqube(primes[i], primes[j]);
        if sqbe > max_val {
            i += 1;
            j = 0;
        } else {
            // println!("{}, {}: {}", primes[i], primes[j], sqbe);
            squbes.push(sqbe);
            j += 1
        }
    }

    squbes.sort();
    squbes
}

fn is_pproof(x: usize) -> bool {
    let units = x % 10;
    let p_digits = vec![1,3,7,9];
    for dig in p_digits.iter() {
        if is_prime(x - units + dig) {return false}
    }

    

    if x % 2 == 0 || x % 5 == 0 {return true}

    let mut potential_primes : Vec<usize> = Vec::new();
    // generate all potential primes (units digit already checked)

    let x_len = x.ilog10() + 1;

    for n in 1..x_len {
        let mag = 10_usize.pow(n);
        let current_dig = (x / mag) % 10;
        // println!("{}, {}", mag, current_dig);

        for dig in 0..10 {
            let pot_prime = x - mag * current_dig + mag * dig;
            // println!("{}", pot_prime);
            potential_primes.push(pot_prime as usize);
        }
    }

    // println!("{:?}", potential_primes);
        
    for pp in potential_primes.iter() {
        if is_prime(*pp) {return false}
    }

    // let digits : Vec<usize> = (0..10).collect();
    true
}