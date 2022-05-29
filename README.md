# 3MUL

During the job interview I got the following task:\
\
For a given array `A` with `u64` elements count how many times the numbers meets the following equation:\
`A[i] = A[j] * A[k] * A[l]` where `i < j < k < l`\
\
For example, for `A = [6, 4, 2, 6, 3, 2, 1]` there are 4 correct sets of numbers `(6, 2, 3, 1), (6, 3, 2, 1), (6, 3, 2, 1), (4, 2, 2, 1)`.
Anothe example, for `A = [1, 1, 1, 1, 1, 1]` there are 15 sets of numbers.

## The solution

The correct solution for the problem is the `O(|A|^2)` algorithm.\
The idea is, change equation to `A[i] / A[j] = A[k] * A[l]`, then iterate over every element, check if given element multiplied by the next elements is present in the HashMap, if it is then get how many times and add it to result. After that, divide every element before with current element and add it to hashmap, increase the counter for such element by one.

```rust
fn quadratic_algorithm(A: &[u64]) -> u64 {
    let mut map: HashMap<u64, u64> = HashMap::new();
    let mut count: u64 = 0;
    for i in 0..A.len() {
        for j in (i + 1)..A.len() {
            let v = A[i] * A[j];
            count += map.get(v).unwrap_or(0);
        }
        for j in 0..i {
            if A[j] % A[i] == 0 {
                let v = A[j] / A[i];
                *map.entry(v).or_insert(0) += 1
            }
        }
    }
    return count;
}
```

## Sub-quadratic solution

After the interview I figured out that there's a way to make a sub-quadratic algorithm. The idea is, we can divide every number by limited amount of numbers. For example 21 can be divided only by `1, 3, 7, 21`. The thing is, we can find those numbers (do factorization) in `O(n^(1/4))` where n is the number, using [Shanks's square forms factorization](https://en.wikipedia.org/wiki/Shanks%27s_square_forms_factorization). When we get all unique combinations of (a, b, c) where a * b * c = value then we can count how many (a, b, c) triplets are there in `O(|A| * log(|A|))` time. The number `17952249695732352000` has the largest amount of unique triples ((a, b, c) dividers) in u64 range, it has `64953936` unique triplests.\
This idea offers `O(|A| * log(|A|) * (max(A) ^ (1/3)))` worst case complexity and `O(|A| * log(|A|) * (max(A) ^ (1/4)))` amortized complexity due to some optimizations.


## Benchmarks

This repository implements both ideas and runs benchmark to compare them. When the input is random the sub-quadratic algorithm always wins for arrays largest than 10k. If the input is designed to be slow for sub-quadratic algorithm then it's faster for larger inputs, starting from ~500k elements.

### 2,500,000 elements benchmark
```
+-----------------------+----------+---------------+---------------------+---------------------+
| Data set              | Elements | Algorithm     | Result              | Execution time (ms) |
+-----------------------+----------+---------------+---------------------+---------------------+
| max_number            | 2500000  | sub-quadratic | 1160732095088676    | 4330875             |
+-----------------------+----------+---------------+---------------------+---------------------+
| max_number            | 2500000  | quadratic     | 1160732095088676    | 44858853            |
+-----------------------+----------+---------------+---------------------+---------------------+
| max_numbers           | 2500000  | sub-quadratic | 333388785437094     | 4842801             |
+-----------------------+----------+---------------+---------------------+---------------------+
| max_numbers           | 2500000  | quadratic     | 333388785437094     | 50667520            |
+-----------------------+----------+---------------+---------------------+---------------------+
| random_numbers        | 2500000  | sub-quadratic | 1102417             | 953                 |
+-----------------------+----------+---------------+---------------------+---------------------+
| random_numbers        | 2500000  | quadratic     | 1102417             | 34199399            |
+-----------------------+----------+---------------+---------------------+---------------------+
| random_unique_numbers | 2500440  | sub-quadratic | 12597818            | 21994               |
+-----------------------+----------+---------------+---------------------+---------------------+
| random_unique_numbers | 2500440  | quadratic     | 12597818            | 53159783            |
+-----------------------+----------+---------------+---------------------+---------------------+
| small_unique_numbers  | 2500000  | sub-quadratic | 47852755            | 18733               |
+-----------------------+----------+---------------+---------------------+---------------------+
| small_unique_numbers  | 2500000  | quadratic     | 47852755            | 21260119            |
+-----------------------+----------+---------------+---------------------+---------------------+
| very_big_numbers      | 2498998  | sub-quadratic | 1264716             | 327919              |
+-----------------------+----------+---------------+---------------------+---------------------+
| very_big_numbers      | 2498998  | quadratic     | 1264716             | 33317428            |
+-----------------------+----------+---------------+---------------------+---------------------+
| very_small_numbers    | 2500000  | sub-quadratic | 2365992826391729681 | 1673                |
+-----------------------+----------+---------------+---------------------+---------------------+
| very_small_numbers    | 2500000  | quadratic     | 2365992826391729681 | 21674296            |
+-----------------------+----------+---------------+---------------------+---------------------+
```

### 500,000 elements benchmark
```
+-----------------------+----------+---------------+--------------------+---------------------+
| Data set              | Elements | Algorithm     | Result             | Execution time (ms) |
+-----------------------+----------+---------------+--------------------+---------------------+
| max_number            | 500000   | sub-quadratic | 1751390284641      | 1399435             |
+-----------------------+----------+---------------+--------------------+---------------------+
| max_number            | 500000   | quadratic     | 1751390284641      | 2255763             |
+-----------------------+----------+---------------+--------------------+---------------------+
| max_numbers           | 500000   | sub-quadratic | 447514594643       | 2010101             |
+-----------------------+----------+---------------+--------------------+---------------------+
| max_numbers           | 500000   | quadratic     | 447514594643       | 2608513             |
+-----------------------+----------+---------------+--------------------+---------------------+
| random_numbers        | 500000   | sub-quadratic | 813313             | 105                 |
+-----------------------+----------+---------------+--------------------+---------------------+
| random_numbers        | 500000   | quadratic     | 813313             | 1374601             |
+-----------------------+----------+---------------+--------------------+---------------------+
| random_unique_numbers | 500605   | sub-quadratic | 443315             | 3541                |
+-----------------------+----------+---------------+--------------------+---------------------+
| random_unique_numbers | 500605   | quadratic     | 443315             | 1498903             |
+-----------------------+----------+---------------+--------------------+---------------------+
| small_unique_numbers  | 500000   | sub-quadratic | 7604992            | 2143                |
+-----------------------+----------+---------------+--------------------+---------------------+
| small_unique_numbers  | 500000   | quadratic     | 7604992            | 838984              |
+-----------------------+----------+---------------+--------------------+---------------------+
| very_big_numbers      | 500000   | sub-quadratic | 265295             | 74254               |
+-----------------------+----------+---------------+--------------------+---------------------+
| very_big_numbers      | 500000   | quadratic     | 265295             | 1341244             |
+-----------------------+----------+---------------+--------------------+---------------------+
| very_small_numbers    | 500000   | quadratic     | 121942304752268154 | 881325              |
+-----------------------+----------+---------------+--------------------+---------------------+
| very_small_numbers    | 500000   | sub-quadratic | 121942304752268154 | 1061                |
+-----------------------+----------+---------------+--------------------+---------------------+
```
