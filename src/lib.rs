use intmap::IntMap;
use itertools::Itertools;
use num::integer::Roots;
use num_integer::binomial;
use num_prime::nt_funcs::factorize64;
use std::cmp;
use std::collections::BTreeMap;

mod fast_hash_set;
use fast_hash_set::FastHashSet;

pub fn quadratic_algorithm(input: &[u64]) -> u64 {
    let mut map: IntMap<u64> = IntMap::new(); // it's faster
    map.insert(0, 0);
    let mut zeros = 0;
    let mut count: u64 = 0;
    for i in 0..input.len() {
        let limit = u64::MAX / cmp::max(1, input[i]);
        for j in (i + 1)..input.len() {
            if input[j] > limit {
                continue; // overflow protection
            }
            let v = input[i] * input[j];
            if let Some(c) = map.get(v) {
                count += c;
            }
        }
        if input[i] == 0 {
            let elements_left = input.len() - i - 1;
            count += zeros * binomial(elements_left, 2) as u64;
            zeros += 1;
            continue;
        }
        for j in 0..i {
            if input[j] % input[i] == 0 {
                let v = input[j] / input[i];
                if let Some(c) = map.get_mut(v) {
                    *c += 1;
                } else {
                    map.insert(v, 1);
                }
            }
        }
    }
    return count;
}

pub fn subquadratic_algorithm(input: &[u64]) -> u64 {
    // first let's count indexes for each number
    let mut indexes: FastHashSet = FastHashSet::new();
    for el in input.iter().unique() {
        indexes.insert(*el, Vec::new());
    }
    for i in 0..input.len() {
        indexes.get_mut(input[i]).unwrap().push(i);
    }

    // now let's iterate over each unique number
    let mut count: u64 = 0;
    if let Some(zeros) = indexes.get(0) {
        count += count_zeros(input.len(), &zeros);
    }
    for value in input.iter().unique() {
        let value = *value;
        if value == 0 {
            continue;
        }
        let factors = factorize64(value); // O(value ^ (1/4))
        let first_dividers = find_first_dividers(value, &factors);
        let l0 = indexes.get(value).unwrap();
        for div1 in first_dividers {
            if let Some(l1) = indexes.get(div1) {
                let value2 = value / div1;
                let second_dividers = find_second_dividers(value, div1, &factors);
                for div2 in second_dividers {
                    if let Some(l2) = indexes.get(div2) {
                        let div3 = value2 / div2;
                        if let Some(l3) = indexes.get(div3) {
                            if div1 == div2 && div2 == div3 {
                                count += count_elements_2((l0, l1));
                            } else if div1 == div2 {
                                count += count_elements_3((l0, l1, l3));
                            } else if div1 == div3 {
                                count += count_elements_3((l0, l1, l2));
                            } else if div2 == div3 {
                                count += count_elements_3((l0, l2, l1));
                            } else {
                                count += count_elements_4((l0, l1, l2, l3));
                            }
                        }
                    }
                }
            }
        }
    }

    return count;
}

// finds all dividers for given number
pub fn find_dividers(value: u64) -> Vec<u64> {
    let factors = factorize64(value);
    let mut vec: Vec<u64> = vec![1];
    for (factor, count) in factors {
        let vec_size = vec.len();
        let mut div = 1;
        for _ in 0..count {
            div *= factor;
            for i in 0..vec_size {
                let value = vec[i] * div;
                vec.push(value);
            }
        }
    }
    for divider in vec.iter_mut() {
        *divider = value / *divider;
    }
    return vec;
}

// finds all dividers for given number in range <number^(1/3), number>
pub fn find_first_dividers(value: u64, factors: &BTreeMap<u64, usize>) -> Vec<u64> {
    let cbrt = (value as f64).cbrt();
    let max = cbrt.powi(2) as u64;
    let mut vec: Vec<u64> = vec![1];
    for (factor, count) in factors {
        let vec_size = vec.len();
        let mut div = 1;
        for _ in 0..*count {
            div *= factor;
            for i in 0..vec_size {
                let value = vec[i] * div;
                if value <= max {
                    vec.push(value);
                }
            }
        }
    }
    for divider in vec.iter_mut() {
        *divider = value / *divider;
    }
    return vec;
}

// finds all dividers for given number in range <1, number^(1/3)>
// should be used after find_first_dividers
pub fn find_second_dividers(
    value: u64,
    first_divider: u64,
    factors: &BTreeMap<u64, usize>,
) -> Vec<u64> {
    let mut minimum_divider = 1;
    if value / first_divider > first_divider {
        let divided = value / first_divider;
        minimum_divider = divided / first_divider;
        if divided % first_divider != 0 {
            minimum_divider += 1; // round up
        }
    }
    let max_divider = (value / first_divider).sqrt() as u64;
    let mut vec1: Vec<u64> = Vec::with_capacity(100);
    let mut vec2: Vec<u64> = Vec::with_capacity(100);
    if minimum_divider == 1 {
        vec2.push(1);
    } else {
        vec1.push(1);
    }
    for (factor, count) in factors {
        let vec1_size = vec1.len();
        let vec2_size = vec2.len();
        let mut div = 1;
        let mut div2 = 1;
        for _ in 0..*count {
            div *= factor;
            if first_divider % div == 0 {
                continue;
            }
            div2 *= factor;
            for i in 0..vec1_size {
                let value1 = vec1[i] * div2;
                if value1 > max_divider {
                    continue;
                }
                if value1 >= minimum_divider {
                    vec2.push(value1);
                } else {
                    vec1.push(value1);
                }
            }
            for i in 0..vec2_size {
                let value1 = vec2[i] * div2;
                if value1 > max_divider {
                    continue;
                }
                vec2.push(value1);
            }
        }
    }
    return vec2;
}

fn count_zeros(len: usize, indexes: &Vec<usize>) -> u64 {
    let mut count = 0;
    let mut zeros: usize = 0;
    for i in indexes {
        let zeros_after = indexes.len() - zeros - 1;
        let non_zero_elements_after = len - i - 1 - zeros_after;
        let c1 = zeros_after * binomial(non_zero_elements_after, 2);
        let c2 = binomial(zeros_after, 2) * non_zero_elements_after;
        let c3 = binomial(zeros_after, 3);
        let c = c1 + c2 + c3;
        count += c;
        zeros += 1;
    }
    count as u64
}

// counts elements for (a, a, a) triplet
fn count_elements_2(lists: (&Vec<usize>, &Vec<usize>)) -> u64 {
    let mut count = 0;
    for i in lists.0 {
        let c1 = lists.1.len() - lists.1.partition_point(|&x| x <= *i);
        count += binomial(c1, 3);
    }
    count as u64
}

// counts elements for (a, a, b) triplet
fn count_elements_3(lists: (&Vec<usize>, &Vec<usize>, &Vec<usize>)) -> u64 {
    let mut count = 0;
    for i in lists.0 {
        let c1 = lists.1.len() - lists.1.partition_point(|&x| x <= *i);
        let c2 = lists.2.len() - lists.2.partition_point(|&x| x <= *i);
        count += binomial(c1, 2) * c2;
    }
    count as u64
}

// counts elements for (a, b, c) triplet
fn count_elements_4(lists: (&Vec<usize>, &Vec<usize>, &Vec<usize>, &Vec<usize>)) -> u64 {
    let mut count = 0;
    for i in lists.0 {
        let c1 = lists.1.len() - lists.1.partition_point(|&x| x <= *i);
        let c2 = lists.2.len() - lists.2.partition_point(|&x| x <= *i);
        let c3 = lists.3.len() - lists.3.partition_point(|&x| x <= *i);
        count += c1 * c2 * c3;
    }
    count as u64
}

// returns the number of unique triples for the number
#[allow(dead_code)]
pub fn count_triplets(value: u64) -> usize {
    let mut count: usize = 0;
    let factors = factorize64(value);
    let first_dividers = find_first_dividers(value, &factors);
    for first_divider in first_dividers {
        count += find_second_dividers(value, first_divider, &factors).len();
    }
    count
}
