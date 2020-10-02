pub fn search(nums: Vec<i32>, target: i32) -> bool {
    if nums.len() == 0 {
        return false;
    }
    let mut left = 0;
    let mut right = nums.len() - 1;

    while left <= right {
        let mid = left + (right - left) / 2;
        if nums[mid] == target {
            return true;
        }
        if nums[mid] == nums[left] {
            left += 1;
            continue;
        }
        //        println!("left = {}, right = {}, mid = {}", left, right, mid);
        if nums[left] < nums[mid] {
            if nums[mid] > target && nums[left] <= target {
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        } else {
            if nums[mid] < target && nums[right] >= target {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
    }
    false
}
