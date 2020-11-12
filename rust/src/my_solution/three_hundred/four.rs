///给定一个二维矩阵，计算其子矩形范围内元素的总和，该子矩阵的左上角为 (row1, col1) ，右下角为 (row2, col2)。
#[allow(non_snake_case)]
pub struct NumMatrix {
    data: Vec<Vec<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumMatrix {
    pub fn new(matrix: Vec<Vec<i32>>) -> Self {
        let y_len = matrix.len();
        if y_len == 0 {
            return Self {
                data: vec![vec![0]],
            };
        }
        let x_len = matrix[0].len();
        let mut dp = vec![vec![0; x_len + 1]; y_len + 1];
        for y in 0..y_len {
            for x in 0..x_len {
                dp[y + 1][x + 1] = dp[y][x + 1] + dp[y + 1][x] - dp[y][x] + matrix[y][x];
            }
        }
        // println!("{:?}", dp);
        Self { data: dp }
    }

    pub fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
        self.data[row2 as usize + 1][col2 as usize + 1]
            - self.data[row2 as usize + 1][col1 as usize]
            - self.data[row1 as usize][col2 as usize + 1]
            + self.data[row1 as usize][col1 as usize]
    }
}
