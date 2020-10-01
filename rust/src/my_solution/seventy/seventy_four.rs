pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    let y_len = matrix.len();
    if y_len == 0 {
        return false;
    }
    let x_len = matrix[0].len();
    if x_len == 0 {
        return false;
    }
    let mut left = 0;
    let mut right = x_len * y_len - 1;
    while left <= right {
        let mid = left + (right - left) / 2;
        //        println!("left = {}, right = {}, mid = {}", left, right, mid);
        let v = matrix[mid / x_len][mid % x_len];
        if v == target {
            return true;
        } else if v > target {
            if mid == 0 {
                return false;
            }
            right = mid - 1;
        } else {
            left = mid + 1;
        }
    }
    false
}
