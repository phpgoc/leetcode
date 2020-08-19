pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    for i in 0..9 {
        for j in 0..9 {
            let cur = board[i][j];
            if cur == '.' {
                continue;
            }
            for in_i in i + 1..9 {
                if board[in_i][j] == cur {
                    return false;
                }
            }
            for in_j in j + 1..9 {
                if board[i][in_j] == cur {
                    return false;
                }
            }
            let mut in_i = if j % 3 == 2 { i + 1 } else { i };
            let mut in_j = if j % 3 == 2 { j - 2 } else { j + 1 };
            //            println!("cur = {}", cur);
            //            println!("i={},j={}", i, j);
            //            println!("in_i_first={},in_j_first={}", in_i, in_j);

            while in_i % 3 != 0 || in_j % 3 != 0 {
                if board[in_i][in_j] == cur {
                    return false;
                }
                //                println!("in_i={},in_j={}", in_i, in_j);

                in_i = if in_j % 3 == 2 { in_i + 1 } else { in_i };
                in_j = if in_j % 3 == 2 { in_j - 2 } else { in_j + 1 };
            }
        }
    }
    true
}
