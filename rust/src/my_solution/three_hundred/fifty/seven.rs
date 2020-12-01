///给定一个非负整数 n，计算各位数字都不同的数字 x 的个数，其中 0 ≤ x < 10n 。
pub fn count_numbers_with_unique_digits(n: i32) -> i32 {
    match n {
        0 => 1,
        1 => 10,
        _ => {
            let mut res = 10;
            let mut base = 9;
            let mut mul = 9;
            for _ in 1..n {
                mul *= base;
                base -= 1;
                res += mul;
            }
            res
        }
    }
}
