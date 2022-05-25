# 3MUL

For a given array `A` with `uint64` elements count how many times the numbers meets the following equation:
```A[i] = A[j] * A[k] * A[l]``` where ```i < j < k < l``` 
For example, for `A = [6, 4, 2, 6, 3, 2, 1]` there are 4 correct sets of numbers `(6, 2, 3, 1), (6, 3, 2, 1), (6, 3, 2, 1), (4, 2, 2, 1)`.