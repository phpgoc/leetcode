///给定一个正整数 n，将其拆分为至少两个正整数的和，并使这些整数的乘积最大化。 返回你可以获得的最大乘积。
pub fn integer_break(n: i32) -> i32 {
    let quotient: u32 = (n / 3) as u32;
    let left = n % 3;
    match (left, quotient) {
        (2, 0) => 1,
        (0, 1) => 2,
        (2, _) => (3_i32.pow(quotient) * 2) as i32,
        (1, _) => (3_i32.pow(quotient - 1) * 4) as i32,
        (0, _) => 3_i32.pow(quotient) as i32,
        _ => 0,
    }
}
