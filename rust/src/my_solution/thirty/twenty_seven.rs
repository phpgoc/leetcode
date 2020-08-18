pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let len = nums.len();
    if len == 0 {
        return 0;
    }
    for i in 0..len {
        let cur_i = len - i - 1;
        if nums[cur_i] == val {
            nums.remove(cur_i);
        }
    }
    nums.len() as i32
}
