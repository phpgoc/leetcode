///你的任务是计算 ab 对 1337 取模，a 是一个正整数，b 是一个非常大的正整数且会以数组形式给出。
pub fn super_pow(a: i32, mut b: Vec<i32>) -> i32 {
    if let Some(last) = b.pop() {
        (my_pow(a, last) * my_pow(super_pow(a, b), 10)) % BASE
    } else {
        1
    }
}
const BASE: i32 = 1337;
fn my_pow(a: i32, b: i32) -> i32 {
    let a = a % BASE;
    let mut res = 1;
    for _ in 0..b {
        res = (res * a) % BASE;
    }
    res
}
