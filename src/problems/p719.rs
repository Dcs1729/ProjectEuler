use std::iter;
use integer_sqrt::IntegerSquareRoot;

pub fn run() {
    
    // let n = 149377284;
    // let s = vec![L,D,D,D,L,D,D,D];
    // let a = split_sum(n, &s);
    // let b = split_display(n, &s);

    // println!("{}: {}", a, b);

    // let x = is_s(1284);
    // println!("x: {}", x);

    // let y: u64 = 15;
    // println!("{}", y.integer_sqrt());

    let z = t(u64::pow(10, 12));
    println!("Total Sum: {}", z)

}

#[derive(Debug, PartialEq, Clone, Copy)]
enum SplitEnum {
    Line,   // Divide
    Dot,    // No divider
}

use SplitEnum::Line as L;
use SplitEnum::Dot as D;

fn split_sum(n: u64, split: &Vec<SplitEnum>) -> u64 { // create split_display function ?
    let digits: usize = (n.checked_ilog10().unwrap_or(0) + 1).try_into().unwrap();
    assert_eq!(digits, split.len() + 1, "{}", format!("\n\nerror in arguments: \n\tn: {}, \n\tsplit array: {:?}\n\n", n, split));

    // split.reversed();
    let segments_iter = split
        .iter()
        .rev()
        .enumerate()
        .filter(|(_i, spl)| spl == &&L)
        .map(|(i, _spl)| i+1);
    
    let segment_1 = iter::once(0).chain(segments_iter.clone());     // 0, {...}
    let segment_2 = segments_iter.chain(iter::once(digits));    // {...}, digits

    let adj_pairs = segment_1.zip(segment_2);  // zipped together into adjacent pairs

    // let mut vec =
    
    adj_pairs
        .map(|(a,b)| (n / 10_u64.pow(a as u32)) % 10_u64.pow((b-a) as u32))
        .sum()

    //     .collect::<Vec<_>>();

    // vec.reverse();
    // vec

    // this commented out code would return it as a Vec<u64> instead of summing the component parts, and reversed so the digits appear as they would in the unsegmented number
    // commented out code also included the "let mut vec = " line above, which should be appended to the adj_pairs line below (this is a bit of a silly solution lol)

    
}

fn split_display(n: u64, split: &Vec<SplitEnum>) -> String {
    let digits: usize = (n.checked_ilog10().unwrap_or(0) + 1).try_into().unwrap(); // repeated code from split_sum check zzz fix later
    assert_eq!(digits, split.len() + 1, "{}", format!("\n\nerror in arguments: \n\tn: {}, \n\tsplit array: {:?}\n\n", n, split));


    let digit_iter = n.to_string();
    let split_iter = split.iter().map(
        |s| match s {
            L => " + ",
            D => ""
        }
    ).chain(iter::once(""));

    digit_iter.chars().zip(split_iter).map(|(d, s)| format!("{}{}", d, s)).collect()
}

fn is_s(n: u64) -> bool { // takes in the square root

    // if n == 1 {return true;} // nvm n=1 is apparently false, we still need to check for n<4 though because single digit numbers with no gaps break the algorithm otherwise
    if n < 4 {return false;}

    let s = u64::pow(n, 2);
    let digits: usize = (s.checked_ilog10().unwrap_or(0) + 1).try_into().unwrap();
    let mut split: Vec<SplitEnum> = vec![D; digits-1];
    // println!("{:?}", split);


    let mut current_index = 0;  // start at -1 so that in the first loop we get current_index = 0 (unless n=1,s=1 then it should just solve immediately with no splitting)
                                // okay nvm i cant do this since an index has to be usize so >= 0 by type, so we instead just check for n=1 manually and skip the first
                                // combination by setting split[0] = L
    split[0] = L;
    let mut go_back = false;
    // let mut check_count = 0;

    'outer: loop {
        let s_sum = split_sum(s, &split);

        // check_count += 1;
        // let s_disp = split_display(s, &split);
        // println!("{}\t({}) {}: {}", check_count, current_index, s_sum, s_disp);

        if s_sum == n {
            let s_disp = split_display(s, &split);
            println!("{}^2 = {}: {}", n, s, s_disp);
            return true;
        }
        else if current_index == digits - 2 { // at the final index, so go back
            go_back = true;
        }
        else { // split further, all split values rightwards of current_index should be Dots
            if s_sum < n {
                split[current_index] = D; // not needed if s_sum > n, because then we want to change LD to LL (and if s_sum < n, we change LD to DL)
            }
            current_index += 1;
            split[current_index] = L;
        }


        if go_back {
            split[current_index] = D;
            'inner: loop {
                if current_index == 0 && split[current_index] == D {
                    break 'outer; // we break completely and return false since we've gone pseudo-lexicographically through all possibilities
                }
                else if split[current_index] == D {
                    current_index -= 1;
                }
                else { // current_index = L
                    split[current_index] = D;
                    current_index += 1;
                    split[current_index] = L; // we change LD to DL
                    break 'inner;
                }
            }
        }

        go_back = false
    }

    return false
    
    // start with the biggest gap: 

    // let gap_start = 3;
    // let gap_end = 4;
    // let mut gap_num = 0;
    // loop {
    //     let a = digits - gap_end - 1;   // number of digits in suffix
    //     let b = digits - gap_start;     // number of digits in prefix
        
    //     gap_num = (n / 10_u64.pow(a as u32)) % 10_u64.pow((b-a) as u32); 

    //     let prefix = n / 10_u64.pow(b as u32);
    //     let suffix = n % 10_u64.pow(a as u32);
    //     // println!("p, s: {}, {}", prefix, suffix);
    //     // println!("pd, sd: {}, {}", p_digits, s_digits);
    //     // if

    //     let mut p_split: Vec<SplitEnum> = vec![D; b-1];
    //     let mut s_split: Vec<SplitEnum> = vec![D; a-1];

    //     if gap_num + prefix + suffix < s {

    //     }
        

        // algorithm:
        // check that gap_num is in the appropriate range

        // if so, calculate pref_min/max and suff_min/max and use those to calculate possible pref/suffix values (save the pair of gap arrangement and sum value)
        // do this for the shorter of the prefix/suffix (in terms of digits (i.e. min(a, b))) and then match against checks for the longer of the prefix/suffix
        // if any match found, 

        // if gap_num not in appropriate range or no matches found, modify the gap by 1 (either increase end or start depending on where in range) and repeat until either
        // a match is found, or gap_start/end out of range

        
    //     break;
    // }

    // println!("digits: {}", digits);

    // gap_num
}

fn t(n: u64) -> u64 { // takes in the square number
    let s_max = n.integer_sqrt();
    (1..s_max + 1)
        .filter(|i| is_s(*i))
        .map(|i| i.pow(2))
        .sum()
}

