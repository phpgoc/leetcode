#[allow(non_snake_case)]
pub fn hammingWeight(n: u32) -> i32 {
    let mut x = n;
    let mut res = 0;
    while x != 0 {
        res += x & 1;
        x >>= 1;
    }
    res as i32
}
