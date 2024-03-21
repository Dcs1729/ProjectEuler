#![allow(unused_imports)]


pub mod utilities;
pub mod problems;

use self::utilities::*;
use self::problems::p59;
use self::problems::p60;
use self::problems::p60_clean;
use self::problems::p200;
use self::problems::p719;
use self::problems::p808;
use self::problems::p816;

use std::time::{Instant, Duration};

fn main() {

    let start_time = Instant::now();

    p60_clean::run();

    let end_time = Instant::now();
    let elapsed_time = end_time - start_time;
    println!("Elapsed time: {} ms", elapsed_time.as_millis());

}
