pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let len = nums.len();
    if (len < 2) {
        return len as i32;
    }
    let mut cur = nums[len - 1];
    for i in 0..len - 1 {
        let cur_i = len - i - 2;
        if nums[cur_i] == cur {
            nums.remove(cur_i);
        } else {
            cur = nums[cur_i];
        }
    }
    nums.len() as i32
}
