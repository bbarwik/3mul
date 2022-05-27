use num_prime::buffer::NaiveBuffer;
use three_mul;

const SIZE: u64 = 500000;

pub fn very_small_numbers() -> Vec<u64> {
    let mut vec: Vec<u64> = Vec::with_capacity(SIZE as usize);
    for i in 0..SIZE {
        vec.push(1 + i % 256);
    }
    return vec;
}

pub fn small_unique_numbers() -> Vec<u64> {
    Vec::from_iter((1..SIZE+1).rev())
}

pub fn very_big_numbers() -> Vec<u64> {
    let mut vec: Vec<u64> = Vec::with_capacity(SIZE as usize);
    for i in 1..=SIZE {
        vec.push(u64::MAX / 2 + i);
    }
    return vec;
}

pub fn max_number() -> Vec<u64> {
    // it's the number with the highest number of triplets
    // it's divided by 2 because IntMap can't support such big number correctly
    // will correct it later, need to find good replacement for IntMap
    const MAX_NUMBER: u64 = 17952249695732352000 / 2; 
    let mut vec: Vec<u64> = Vec::with_capacity(SIZE as usize);
    vec.push(MAX_NUMBER);
    let dividers = three_mul::find_dividers(MAX_NUMBER);
    while vec.len() < SIZE as usize {
        vec.extend(&dividers);
    }
    vec.truncate(SIZE as usize);
    vec.sort();
    vec.reverse();
    return vec;
}

pub fn max_numbers() -> Vec<u64> {
    // it's the number with the highest number of triplets
    const MAX_NUMBERS: [u64; 6] = [
        17952249695732352000 / 2, // limited because IntMap fails
        17820842462599176000 / 2,
        18053332182757872000 / 2,
        15334213281771384000 / 2,
        16082223685760232000 / 2,
        18020081695100284800 / 2,
    ];
    let mut vec: Vec<u64> = Vec::with_capacity(SIZE as usize);
    'generator: loop {
        for number in MAX_NUMBERS {
            let dividers = three_mul::find_dividers(number);
            vec.extend(&dividers);
            if vec.len() > SIZE as usize {
                break 'generator;
            }
        }
    }
    vec.truncate(SIZE as usize);
    vec.sort();
    vec.reverse();
    return vec;
}

pub fn random_unique_numbers() -> Vec<u64> {
    let mut generator = NaiveBuffer::new();
    let primes: Vec<u64> = generator.primes(SIZE / 100).cloned().collect();
    let mut vec: Vec<u64> = Vec::from_iter(1..1000);
    let mut i = 0;
    while vec.len() < SIZE as usize {
        let vec_len = vec.len();
        for prime in &primes {
            let el = vec[i % vec_len];
            i += 1;
            if u64::MAX / el < *prime {
                continue;
            }
            vec.push(el * prime);
        }
        vec.sort();
        vec.dedup();
    }
    vec.reverse();
    return vec;
}

pub fn random_numbers() -> Vec<u64> {
    let mut generator = NaiveBuffer::new();
    let primes: Vec<u64> = generator.primes(SIZE / 10).cloned().collect();
    let mut vec: Vec<u64> = Vec::from_iter(1..1000);
    let mut i = 0;
    while vec.len() < SIZE as usize {
        let vec_len = vec.len();
        for prime in &primes {
            let el = vec[i % vec_len];
            i += 1;
            if u64::MAX / el < *prime {
                continue;
            }
            vec.push(el * prime);
        }
    }
    vec.truncate(SIZE as usize);
    vec.reverse();
    return vec;
}

pub fn get_data_sets() -> Vec<(&'static str, Vec<u64>)> {
    vec![
        ("very_small_numbers", very_small_numbers()),
        ("very_big_numbers", very_big_numbers()),
        ("small_unique_numbers", small_unique_numbers()),
        ("max_number", max_number()),
        ("max_numbers", max_numbers()),
        ("random_numbers", random_numbers()),
        ("random_unique_numbers", random_unique_numbers()),
    ]
}
