use std::collections::HashMap;

pub fn run() {
    let mut digits_map: HashMap<[u8;10], Vec<usize>> = HashMap::new();
    let threshhold = 3;

    for i in 1..(1*usize::pow(10, 4)) {
        let i_key = get_digits_key(u128::pow(i.try_into().unwrap(), 3));
        insert_num(&mut digits_map, i_key, i);
    }

    let mut perms_vec = digits_map
        .iter()
        .filter(|(_, v)| v.len() >= threshhold)
        .collect::<Vec<(&[u8;10], &Vec<usize>)>>();

    perms_vec
        .sort_by(|(_, v1), (_, v2)| {
            let l1 = v1.len(); let l2 = v2.len();
            l2.cmp(&l1)
        });

    println!("There are {} sets of permutations of length at least {}.", perms_vec.len(), threshhold);
    
    perms_vec
        .iter()
        .take(20)
        .for_each(|(k, v)| println!("{}: \t{:?} - {:?}", v.len(), k, v));

    //     .max()
    //     .unwrap();
    
    // println!("Highest number of perms: {}", max_perms);

}

fn get_digits_key(n: u128) -> [u8;10] {
    let mut digit_count: [u8; 10] = [0; 10]; // digit_count[d] should be the number of occurences of digit d in n
    let digits = String::from("0123456789");
    for digit in n.to_string().chars() {
        match digits.find(digit) {
            Some(i) => digit_count[i] += 1,
            None => panic!("unexpected char found"),
        }
    }
    digit_count
}

fn insert_num(map: &mut HashMap<[u8;10], Vec<usize>>, key: [u8; 10], num: usize) {
    match map.get_mut(&key) {
        Some(v) => v.push(num),
        None => {
            map.insert(key, vec![num]);
        },
    };
}