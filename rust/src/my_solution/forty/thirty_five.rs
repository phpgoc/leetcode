pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    let len = nums.len();
    if len == 0 {
        return 0;
    }
    if nums[0] >= target {
        return 0;
    }
    if nums[len - 1] < target {
        return len as i32;
    }
    let mut l = 0;
    let mut r = len - 1;
    let mut result = 0;
    while l < r - 1 {
        result = l + (r - l) / 2;
        if nums[result] == target {
            return result as i32;
        } else if nums[result] > target {
            r = result;
        } else {
            l = result;
        }
    }
    (l + 1) as i32
}
