pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    for i in 0..nums.len() {
        while (0 < nums[i]
            && nums[i] < nums.len() as i32
            && nums[i] != nums[(nums[i] - 1) as usize])
        {
            let swap_to = (nums[i] - 1) as usize;
            nums.swap(i, swap_to);
        }
    }
    for i in 0..nums.len() {
        if nums[i] != (i + 1) as i32 {
            return (i + 1) as i32;
        }
    }
    return (nums.len() + 1) as i32;
}
