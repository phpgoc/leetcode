///给定一个无序的数组 nums，将它重新排列成 nums[0] < nums[1] > nums[2] < nums[3]... 的顺序。
pub fn wiggle_sort(nums: &mut Vec<i32>) {
    nums.sort();
    let len = nums.len();
    nums[0..len / 2 + len % 2].reverse();
    for i in 0..len / 2 {
        nums.insert(i * 2 + 1, nums[len - 1]);
    }
    nums.truncate(len);
}
