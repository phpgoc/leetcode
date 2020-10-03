pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
    let len = heights.len();
    let mut result: i32 = 0;
    let mut stack = vec![];
    let mut max_height = -1;
    let mut cur_index = 0;
    let mut poped = false;
    for (i, &v) in heights.iter().enumerate() {
        //        println!(
        //            "p----{:?},result = {},max_height = {}, max_index = {} , i = {},v = {}",
        //            stack, result, max_height, cur_index, i, v
        //        );
        poped = false;
        while max_height > v {
            result = result.max((i - cur_index) as i32 * max_height);
            match stack.pop() {
                Some((index, h)) => {
                    cur_index = index;
                    max_height = h;
                    poped = true;
                }
                None => {
                    max_height = -1;
                }
            }
        }

        if max_height != -1 {
            stack.push((cur_index, max_height));
        }
        stack.push((cur_index, v));
        if poped {
            cur_index = i + 1;
        } else {
            cur_index += 1;
        }
        max_height = v;
        //        println!(
        //            "{:?},result = {},max_height = {}, cur_index = {} , i = {},v = {}",
        //            stack, result, max_height, cur_index, i, v
        //        );
    }
    while let Some((i, h)) = stack.pop() {
        max_height = h;
        result = result.max((len - i) as i32 * h);
    }
    result
}
