pub mod eight;
pub mod four;
#[cfg(test)]
mod tests;
pub mod three;
///给定一个整数数组 nums，其中恰好有两个元素只出现一次，其余所有元素均出现两次。 找出只出现一次的那两个元素。
pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
    let mut bitmask = 0;
    for &i in &nums {
        bitmask ^= i;
    }
    let diff = bitmask & -bitmask;
    let mut x = 0;
    for &i in &nums {
        if i & diff == 0 {
            x ^= i;
        }
    }
    vec![x, bitmask ^ x]
}
