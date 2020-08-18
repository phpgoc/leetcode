pub fn next_permutation(nums: &mut Vec<i32>) {
    let len = nums.len();
    let mut cur = len - 1;
    while cur > 0 {
        if nums[cur - 1] < nums[cur] {
            if cur == len - 1 {
                nums.swap(cur - 1, cur);
                return;
            }
            let mut i = 0;
            let mut j = cur + 1;
            while j < len {
                if nums[j] <= nums[cur - 1] {
                    nums.swap(j - 1, cur - 1);
                    break;
                }
                j += 1;
            }
            //            println!("cur = {}", cur);
            //            println!("J = {}", j);
            if j == len {
                nums.swap(cur - 1, len - 1);
            }
            while cur + i < len - i - 1 {
                nums.swap(cur + i, len - i - 1);
                i += 1;
            }

            return;
        }
        cur -= 1;
    }
    let mut i = 0;
    while i < len - i - 1 {
        nums.swap(i, len - i - 1);
        i += 1;
    }
}
