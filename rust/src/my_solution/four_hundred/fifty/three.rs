///给定一个长度为 n 的 非空 整数数组，每次操作将会使 n - 1 个元素增加 1。找出让数组所有元素相等的最小操作次数。
pub fn min_moves(nums: Vec<i32>) -> i32 {
    let mut res = 0;
    let mut min = nums[0];
    for (k, &v) in nums[1..].iter().enumerate() {
        if v >= min {
            res += v - min;
        } else {
            res += (k as i32 + 1) * (min - v);
            min = v;
        }
    }
    res
}
