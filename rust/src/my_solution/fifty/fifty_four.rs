pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    let mut up = 0;
    let mut left = 0;
    let mut result = vec![];
    if matrix.len() == 0 || matrix[0].len() == 0 {
        return result;
    }
    let mut down = matrix.len() - 1;
    let mut right = matrix[0].len() - 1;

    while up <= down && left <= right {
        for i in left..=right {
            result.push(matrix[up][i]);
        }
        for i in up + 1..=down {
            result.push(matrix[i][right]);
        }
        if up != down {
            for i in (left..right).rev() {
                result.push(matrix[down][i]);
            }
        }
        if left != right {
            for i in (up + 1..down).rev() {
                result.push(matrix[i][left]);
            }
        }
        if down == 0 || right == 0 {
            break;
        }
        up += 1;
        down -= 1;
        left += 1;
        right -= 1;
    }
    result
}
