pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
    let len = matrix.len();
    for y in 0..(len / 2 + len % 2) {
        for x in 0..(len / 2) {
            let tmp = matrix[y][x];
            matrix[y][x] = matrix[len - 1 - x][y];
            matrix[len - 1 - x][y] = matrix[len - 1 - y][len - 1 - x];
            matrix[len - 1 - y][len - 1 - x] = matrix[x][len - 1 - y];
            matrix[x][len - 1 - y] = tmp;
            //            println!("{:?}", matrix);
            //            println!("x = {},y = {}", x, y);
        }
    }
}
