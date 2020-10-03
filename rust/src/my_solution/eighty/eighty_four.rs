pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
    let len = heights.len();
    let mut result: i32 = 0;
    let mut stack = vec![];
    let mut max_height = -1;
    let mut to_i = 0;
    let mut poped = false;
    for (i, &v) in heights.iter().enumerate() {
        //        println!(
        //            "p----{:?},result = {},max_height = {}, max_index = {} , i = {},v = {}",
        //            stack, result, max_height, cur_index, i, v
        //        );
        poped = false;
        while max_height > v {
            result = result.max((i - to_i) as i32 * max_height);
            match stack.pop() {
                Some((to_i_, height)) => {
                    poped = true;
                    if height <= v {
                        stack.push((to_i_, height));
                        break;
                    }
                    to_i = to_i_;
                    max_height = height;
                }
                None => {
                    max_height = -1;
                }
            }
        }

        if poped {
            stack.push((to_i, v));
        } else {
            stack.push((i, v));
            to_i = i;
        }
        max_height = v;
        //        println!(
        //            "{:?},result = {},max_height = {}, cur_i = {} , i = {},v = {}",
        //            stack, result, max_height, cur_i, i, v
        //        );
    }
    while let Some((to_i, h)) = stack.pop() {
        result = result.max((len - to_i) as i32 * h);
    }
    result
}
