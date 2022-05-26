use num_prime::buffer::NaiveBuffer;
use three_mul;

const SIZE: u64 = 250000;

pub fn very_small_numbers() -> Vec<u64> {
    let mut vec: Vec<u64> = Vec::with_capacity(SIZE as usize);
    for i in 0..SIZE {
        vec.push(1 + i % 256);
    }
    return vec;
}

pub fn very_big_numbers() -> Vec<u64> {
    let mut vec: Vec<u64> = Vec::with_capacity(SIZE as usize);
    for i in 1..=SIZE {
        vec.push(u64::MAX - i);
    }
    return vec;
}

pub fn max_number() -> Vec<u64> {
    // it's the number with the highest number of triplets
    const MAX_NUMBER: u64 = (2 * 3 * 5 * 7 * 11 * 13 * 17 * 19 * 21 * 2 * 3 * 3)
        * (2 * 3 * 5 * 7 * 11 * 13 * 17 * 19 * 21 * 2 * 3 * 3);
    let mut vec: Vec<u64> = Vec::with_capacity(SIZE as usize);
    vec.push(MAX_NUMBER);
    let dividers = three_mul::find_dividers(MAX_NUMBER);
    while vec.len() < SIZE as usize {
        vec.extend(&dividers);
    }
    vec.truncate(SIZE as usize);
    return vec;
}

pub fn multiple_max_number() -> Vec<u64> {
    // it's the number with the highest number of triplets
    let mut vec: Vec<u64> = Vec::with_capacity(SIZE as usize);
    let mut generator = NaiveBuffer::new();
    let primes : Vec<u64> = generator.primes(1000).cloned().collect();
    let mut primes_index = 0;
    let mut number = 1;
    loop {
        if u64::MAX / number > primes[primes_index] {
            number *= primes[primes_index];
            primes_index = (primes_index + 1) % primes.len();
        } else {
            let dividers = three_mul::find_dividers(number);
            vec.extend(&dividers);
            if vec.len() > SIZE as usize {
                break;
            }
            number = 1;
        }
    }
    vec.truncate(SIZE as usize);
    vec.sort();
    vec.reverse();
    return vec;
}

pub fn random_numbers() -> Vec<u64> {
    let mut generator = NaiveBuffer::new();
    let primes : Vec<u64> = generator.primes(1000).cloned().collect();
    let mut vec: Vec<u64> = Vec::from_iter(1..1000);
    let max = u64::MAX / 1000;
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
        ("max_number", max_number()),
        ("multiple_max_number", multiple_max_number()),
        ("random_numbers", random_numbers())
    ]
}
