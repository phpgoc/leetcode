pub fn can_jump(nums: Vec<i32>) -> bool {
    let mut max_touch = 0;
    let len = nums.len();
    let target = len - 1;
    for i in 0..len - 1 {
        if i > max_touch {
            return false;
        }
        max_touch = max_touch.max(nums[i] as usize + i);
        //        println!("i = {},max_touch = {}", i, max_touch);
    }

    max_touch >= target
}
