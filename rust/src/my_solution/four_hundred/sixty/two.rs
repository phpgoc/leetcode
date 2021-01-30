///给定一个非空整数数组，找到使所有数组元素相等所需的最小移动数，其中每次移动可将选定的一个元素加1或减1。 您可以假设数组的长度最多为10000。
pub fn min_moves2(mut nums: Vec<i32>) -> i32 {
    nums.sort();
    let mid = nums[nums.len() / 2];
    let mut res = 0;
    for i in nums {
        res += (i - mid).abs();
    }
    res as i32
}
