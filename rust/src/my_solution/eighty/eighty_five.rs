pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
    let y_len = matrix.len();
    if y_len == 0 {
        return 0;
    }
    let x_len = matrix[0].len();
    let mut ans = 0;

    let mut left_and_up = vec![vec![(0, 0); x_len]; y_len];
    for y in 0..y_len {
        for x in 0..x_len {
            let mut left_to = 0;
            let mut up_to = 0;
            for p_x in (0..=x).rev() {
                if matrix[y][p_x] == '1' {
                    left_to += 1;
                } else {
                    break;
                }
            }
            for p_y in (0..=y).rev() {
                if matrix[p_y][x] == '1' {
                    up_to += 1;
                } else {
                    break;
                }
            }
            left_and_up[y][x] = (left_to, up_to);
        }
    }
    //    for i in &left_and_up {
    //        println!("{:?}", i);
    //    }
    for y in (0..y_len).rev() {
        for x in (0..x_len).rev() {
            if matrix[y][x] == '1' {
                let mut cur_ans = 0;
                let left = left_and_up[y][x].0;
                let up = left_and_up[y][x].1;
                if left * up <= ans {
                    continue;
                }
                cur_ans = recursive(x, y, &x, &y, left, up, &matrix, &left_and_up, cur_ans);
                ans = ans.max(cur_ans);
            }
        }
    }
    ans
}

fn recursive(
    x: usize,
    y: usize,
    from_x: &usize,
    from_y: &usize,
    mut max_left: i32,
    mut max_up: i32,
    matrix: &Vec<Vec<char>>,
    left_and_up: &Vec<Vec<(i32, i32)>>,
    mut cur_ans: i32,
) -> i32 {
    //    println!(
    //        "x = {} ,y = {},from_x ={},from_y = {}, cur_ans = {}",
    //        x, y, from_x, from_y, cur_ans
    //    );

    let left = left_and_up[y][x].0;
    let up = left_and_up[y][x].1;
    if left == 0 {
        return cur_ans;
    }
    max_left = max_left.min(left);
    max_up = max_up.min(up);
    let from_to_x = (from_x - x) as i32;
    let from_to_y = (from_y - y) as i32;
    cur_ans = cur_ans.max((from_to_x + 1) * (from_to_y + max_up));
    cur_ans = cur_ans.max((from_to_y + 1) * (from_to_x + max_left));
    //    println!("cur_ans = {}", cur_ans);
    if max_left == 1 || max_up == 1 || x == 0 || y == 0 {
        return cur_ans;
    }
    cur_ans = recursive(
        x - 1,
        y - 1,
        from_x,
        from_y,
        max_left - 1,
        max_up - 1,
        matrix,
        left_and_up,
        cur_ans,
    );
    cur_ans
}
