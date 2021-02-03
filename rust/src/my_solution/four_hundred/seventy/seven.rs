///两个整数的 汉明距离 指的是这两个数字的二进制数对应位不同的数量。
///
///计算一个数组中，任意两个数之间汉明距离的总和。
pub fn total_hamming_distance(nums: Vec<i32>) -> i32 {
    let mut res = 0;
    let len = nums.len() as i32;
    if len == 0 {
        return 0;
    }
    let max = *nums.iter().max().unwrap();
    let mut high_bit_one = 1;
    while high_bit_one <= max {
        let mut count = 0;
        for &i in nums.iter() {
            if high_bit_one & i == 0 {
                count += 1;
            }
        }
        res += count * (len - count);
        high_bit_one <<= 1;
    }
    res
}
