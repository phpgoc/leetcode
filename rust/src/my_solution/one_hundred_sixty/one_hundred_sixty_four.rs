pub fn maximum_gap(nums: Vec<i32>) -> i32 {
    let len = nums.len();
    if len < 2 {
        return 0;
    }
    let mut nums = nums;
    nums.sort();
    let mut max = nums[1] - nums[0];
    for i in 2..len {
        max = max.max(nums[i] - nums[i - 1]);
    }
    max
}
