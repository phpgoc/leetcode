use std::collections::HashSet;

pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
    let mut column = HashSet::new();
    let y_len = matrix.len();
    if y_len == 0 {
        return;
    }
    let x_len = matrix[0].len();
    let mut do_row = false;
    for y in 0..y_len {
        do_row = false;
        for x in 0..x_len {
            if matrix[y][x] == 0 {
                do_row = true;
                column.insert(x);
            }
        }
        if do_row {
            for i in 0..x_len {
                matrix[y][i] = 0;
            }
        }
    }
    for i in column {
        for y in 0..y_len {
            matrix[y][i] = 0;
        }
    }
}
