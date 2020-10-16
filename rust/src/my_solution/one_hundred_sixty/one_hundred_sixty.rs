pub fn find_peak_element(nums: Vec<i32>) -> i32 {
    let mut res = 0;
    if nums.len() < 2 {
        return res as i32;
    }
    for i in 1..nums.len() {
        if nums[i] > nums[i - 1] {
            res = i;
        } else {
            break;
        }
    }
    res as i32
}
