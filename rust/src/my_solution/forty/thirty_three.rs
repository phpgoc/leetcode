pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    let len = nums.len();
    let mut l = 0;
    let mut r = len - 1;
    while l <= r {
        let mid = (l + r) / 2;
        if nums[mid] == target {
            return mid as i32;
        } else if nums[mid] < nums[r] {
            if nums[mid] < target && target <= nums[r] {
                l = mid + 1;
            } else {
                if mid == 0 {
                    return -1;
                }
                r = mid - 1;
            }
        } else if (nums[l] <= target && target < nums[mid]) {
            r = mid - 1;
        } else {
            l = mid + 1;
        }
    }
    -1
}
