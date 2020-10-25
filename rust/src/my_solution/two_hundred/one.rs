pub fn range_bitwise_and(m: i32, n: i32) -> i32 {
    let mut m = m;
    let mut n = n;
    let mut i = 0;
    while m < n {
        m >>= 1;
        n >>= 1;
        i += 1;
    }
    m <<= i;
    m
}
