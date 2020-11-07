///给定一个非负整数 num，反复将各个位上的数字相加，直到结果为一位数。
pub fn add_digits(num: i32) -> i32 {
    (num - 1) % 9 + 1
}
