use std::fs;



pub fn run() {
    let filepath = "euler_files\\0059_cipher.txt";
    let ciphertext = fs::read_to_string(filepath).unwrap_or("Error".to_string()); // 1455 chars

    let ascii_vals: Vec<u8> = ciphertext
        .split(",")
        .map(|n| n.parse::<u8>().unwrap())
        .collect();

    // println!("{} {:?}", ascii_vals.len(), ascii_vals);
    // println!("{}", get_text(&ascii_vals));
    // println!("{}", 107 ^ 42);

    // let key: Vec<u8> = vec![1,2,3];
    // let enc = encrypt(&ascii_vals, &key);
    // let dec = encrypt(&enc, &key);

    // let n = 10;
    // let asc_t: Vec<u8> = ascii_vals.iter().take(n).copied().collect();
    // let enc_t: Vec<u8> = enc       .iter().take(n).copied().collect();
    // let dec_t: Vec<u8> = dec       .iter().take(n).copied().collect();
    

    // println!("{:?}\n{:?}\n{:?}", asc_t, enc_t, dec_t);

    let lowercases = 97..=122; // 110..=112; // 
    let mut keys = Vec::<Vec<u8>>::new();

    for a in lowercases.clone() {
        for b in lowercases.clone() {
            for c in lowercases.clone() {
                keys.push(vec![a, b, c]);
            }
        }
    }
    // println!("{:?}", keys);
    println!("{}", keys.len());

    let mut plaintexts: Vec<(Vec<u8>, usize)> = keys
        .iter()
        .map(|key| (key, encrypt(&ascii_vals, key)))
        .map(|(key, text)| (key.clone(), {
                text
                    .iter()
                    .filter(|i| 
                        (**i >= 65 && **i <= 90) || 
                        (**i >= 97 && **i <= 112) ||
                        **i == 32
                        
                    )
                    .count()
            }))
        .collect();
    
    plaintexts.sort_by(|(_, a), (_, b)| b.cmp(a));
    // plaintexts.reverse();

    let lowest: Vec<_> = plaintexts
        .iter()
        .rev()
        .take(20)
        .rev()
        .collect();

    let highest: Vec<_> = plaintexts
        .iter()
        .take(20)
        .collect();

    for h in &highest {
        println!("{:?}", *h);
    }
    println!("...");
    for l in &lowest {
        println!("{:?}", *l);
    }


    for i in 0..10 {
        let (key, count) = &plaintexts[i];
        println!("{} {} {:?} {}", i, count, key, get_text(key));
        

        let new_plaintext = get_text(&encrypt(&ascii_vals, &key));
        println!("{}\n", new_plaintext);
    }

    let new_plaintext = &encrypt(&ascii_vals, &vec![101, 120, 112]);

    let vals_sum: usize = new_plaintext
        .iter()
        .copied()
        .map(|v| v as usize)
        .sum();

    println!("{}", get_text(new_plaintext));
    println!("{}", vals_sum);

    // println!("{}\n", new_plaintext);


    // encrypt each key and count the number of chars that are letters, check for >90% or smth
}



fn get_text(ascii: &Vec<u8>) -> String {
    ascii
        .iter()
        .map(|n| *n as char)
        .collect()
}

fn encrypt(vals: &Vec<u8>, key: &Vec<u8>) -> Vec<u8> {
    let mut key_iter = key.iter().cycle();

    vals
        .iter()
        .map(|n| *n ^ *key_iter.next().unwrap())
        .collect()
}