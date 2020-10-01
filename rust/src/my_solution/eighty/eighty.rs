pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut for_delete = 0;
    let mut cur_v = std::i32::MIN;
    let len = nums.len();
    let mut count = 0;
    for i in (0..len).rev() {
        if nums[i] == cur_v {
            count += 1;
        } else {
            count = 1;
            cur_v = nums[i];
            for_delete = i;
        }
        if count > 2 {
            nums.remove(for_delete);
            for_delete -= 1;
        }
    }
    nums.len() as i32
}
