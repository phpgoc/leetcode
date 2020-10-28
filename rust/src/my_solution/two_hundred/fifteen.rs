///在未排序的数组中找到第 k 个最大的元素。请注意，你需要找的是数组排序后的第 k 个最大的元素，而不是第 k 个不同的元素。
pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
    let mut nums = nums;
    let len = nums.len();

    recursive(&mut nums, k as usize - 1, 0, len - 1)
}

fn recursive(nums: &mut Vec<i32>, k: usize, left: usize, right: usize) -> i32 {
    let position = partition(nums, left, right);
    // println!("{:?},position = {}", nums, position);
    if position == k {
        return nums[k];
    } else if position > k {
        return recursive(nums, k, left, position - 1);
    } else {
        return recursive(nums, k, position + 1, right);
    }
}
extern "C" {
    fn srand() -> u32;
    fn rand() -> u32;
}
fn partition(nums: &mut Vec<i32>, left: usize, right: usize) -> usize {
    if left == right {
        return left;
    }
    let random = unsafe {
        srand();
        rand()
    } as usize
        % (right - left + 1);
    nums.swap(left, left + random);
    let mut j = left;
    for cur in left + 1..=right {
        if nums[cur] > nums[left] {
            j += 1;
            nums.swap(j, cur);
        }
    }
    nums.swap(left, j);
    j
}
