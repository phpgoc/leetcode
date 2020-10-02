pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
    let y = board.len();
    if y == 0 {
        return false;
    }
    let x = board[0].len();
    let mut in_path = vec![];
    let str_vec: Vec<_> = word.chars().collect();

    for y_i in 0..y {
        for x_i in 0..x {
            if dfs(
                x_i as i32,
                y_i as i32,
                x,
                y,
                &str_vec,
                &board,
                0,
                &mut in_path,
            ) {
                return true;
            } else {
                in_path.clear();
            }
        }
    }

    false
}

fn in_area(x: i32, y: i32, x_len: usize, y_len: usize) -> bool {
    x >= 0 && y >= 0 && x < x_len as i32 && y < y_len as i32
}

fn dfs(
    x: i32,
    y: i32,
    x_len: usize,
    y_len: usize,
    str_vec: &Vec<char>,
    vec: &Vec<Vec<char>>,
    index: usize,
    in_path: &mut Vec<i32>,
) -> bool {
    let dir = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
    if !in_area(x, y, x_len, y_len) {
        return false;
    }
    if vec[y as usize][x as usize] != str_vec[index] {
        return false;
    }

    if in_path.contains(&(y * x_len as i32 + x)) {
        return false;
    }
    if index == str_vec.len() - 1 {
        return true;
    }
    for (x_add, y_add) in dir {
        in_path.push(y * x_len as i32 + x);
        if (dfs(
            x + x_add,
            y + y_add,
            x_len,
            y_len,
            str_vec,
            vec,
            index + 1,
            in_path,
        )) {
            return true;
        } else {
            in_path.pop();
        }
    }
    false
}
