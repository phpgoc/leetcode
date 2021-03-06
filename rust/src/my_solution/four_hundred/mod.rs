///在无限的整数序列 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, ...中找到第 n 个数字。
pub fn find_nth_digit(n: i32) -> i32 {
    let n = n as i64;
    let mut bit_len = 1;
    let mut res = 0;
    let mut base = 9;
    loop {
        res += base * bit_len;
        if res >= n {
            let quotient = (base * bit_len + n - res - 1) / bit_len;
            let left = (base * bit_len + n - res - 1) % bit_len;
            let mut num = 10_i64.pow(bit_len as u32 - 1) + quotient;
            println!("quotient = {},num = {},left = {}", quotient, num, left);
            for _ in 1..bit_len - left {
                num /= 10;
            }
            return num as i32 % 10;
        } else {
            base *= 10;
            bit_len += 1;
        }
    }
}
pub mod eighty;
pub mod fifteen;
pub mod fifty;
pub mod five;
pub mod forty;
pub mod fourteen;
pub mod nine;
pub mod nineteen;
pub mod ninty;
pub mod one;
pub mod seventeen;
pub mod seventy;
pub mod six;
pub mod sixteen;
pub mod sixty;
pub mod ten;
#[cfg(test)]
mod tests;
pub mod thirteen;
pub mod thirty;
pub mod twelve;
pub mod twenty;
pub mod two;
