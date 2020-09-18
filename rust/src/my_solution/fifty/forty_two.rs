pub fn trap(height: Vec<i32>) -> i32 {
    let height_len = height.len();
    if height_len <= 1 {
        return 0;
    }
    let mut cur_len = height[0];
    let mut cur_i = 0;
    let mut total = 0;
    let mut cur_v = 0;

    for i in 1..height_len {
        if height[i] >= cur_len {
            total += cur_v;
            cur_len = height[i];
            cur_i = i;
            cur_v = 0;
        //            println!("i = {},total = {}", i, total);
        } else {
            cur_v += cur_len - height[i];
        }
    }
    if height[height_len - 1] < cur_len {
        let mut recursive_vec = height[cur_i..].to_vec();
        recursive_vec.reverse();
        total += trap(recursive_vec);
    }
    total
}
