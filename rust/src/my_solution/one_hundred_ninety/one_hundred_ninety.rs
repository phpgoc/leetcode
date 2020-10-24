pub fn reverse_bits(x: u32) -> u32 {
    let mut x = x;
    let mut res = 0;

    for _ in 0..32 {
        res <<= 1;
        res += x & 1;
        x >>= 1;
    }
    res
}
