use std::fs;

pub fn run() {
    let filepath = "euler_files\\0089_roman.txt";
    let roman_txt = fs::read_to_string(filepath).unwrap_or("Error".to_string()); // 1455 chars

    let roman_nums: Vec<&str> = roman_txt
    .split("\n")
    .collect();

    println!("{:#?}", roman_nums.len());

    let mut chars_saved: usize = 0;
    for r in roman_nums {
        let r_reduce = dec_to_roman(roman_to_dec(r));
        if *r != r_reduce {
            println!("{} | {}", r, r_reduce);
            chars_saved += r.chars().count() - r_reduce.chars().count();
        }
    }

    println!("{}", chars_saved);
}

fn roman_to_dec(roman: &str) -> usize {
    String::from(roman)
        .replace("IV", "A")
        .replace("IX", "B")
        .replace("XL", "E")
        .replace("XC", "F")
        .replace("CD", "G")
        .replace("CM", "H")
        .chars()
        .map(|c| {
            match c {
                'I' => 1,
                'A' => 4,
                'V' => 5,
                'B' => 9,
                'X' => 10,
                'E' => 40,
                'L' => 50,
                'F' => 90,
                'C' => 100,
                'G' => 400,
                'D' => 500,
                'H' => 900,
                'M' => 1000,
                _   => panic!("unexpected character"),
            }
        })
        .sum()
}

fn dec_to_roman(dec: usize) -> String {
    let mut roman = String::new();
    let mut d = dec;
    while d > 0 {
        match d {
            x if x >= 1000 => {roman.push('M'); d -= 1000;},
            x if x >= 900 => {roman.push('H'); d -= 900;},
            x if x >= 500 => {roman.push('D'); d -= 500;},
            x if x >= 400 => {roman.push('G'); d -= 400;},
            x if x >= 100 => {roman.push('C'); d -= 100;},
            x if x >= 90 => {roman.push('F'); d -= 90;},
            x if x >= 50 => {roman.push('L'); d -= 50;},
            x if x >= 40 => {roman.push('E'); d -= 40;},
            x if x >= 10 => {roman.push('X'); d -= 10;},
            x if x >= 9 => {roman.push('B'); d -= 9;},
            x if x >= 5 => {roman.push('V'); d -= 5;},
            x if x >= 4 => {roman.push('A'); d -= 4;},
            x if x >= 1 => {roman.push('I'); d -= 1;},
            _ => panic!("unexpected value of d"),
        }
    }
    roman
        .replace("A", "IV")
        .replace("B", "IX")
        .replace("E", "XL")
        .replace("F", "XC")
        .replace("G", "CD")
        .replace("H", "CM")
}

// Custom Numerals Ruleset:
// I        1
// A (IV)   4
// V        5
// B (IX)   9
// X        10
// E (XL)   40
// L        50
// F (XC)   90
// C        100
// G (CD)   400
// D        500
// H (CM)   900
// M        1000