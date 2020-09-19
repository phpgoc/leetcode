pub fn jump(nums: Vec<i32>) -> i32 {
    let nums_len = nums.len();
    let mut until = nums_len - 1;
    let mut step = 0;
    while until > 0 {
        for i in 0..=until {
            if nums[i] as usize + i >= until {
                until = i;
                step += 1;
                break;
            }
        }
    }
    step
}
