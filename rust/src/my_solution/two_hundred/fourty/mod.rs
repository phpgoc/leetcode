pub mod one;
#[cfg(test)]
mod tests;
pub mod two;
///编写一个高效的算法来搜索 m x n 矩阵 matrix 中的一个目标值 target。该矩阵具有以下特性：
//
// 每行的元素从左到右升序排列。
// 每列的元素从上到下升序排列。
//
// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/search-a-2d-matrix-ii
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    let (m, n) = match matrix.first() {
        Some(t) => (matrix.len(), t.len()),
        _ => {
            return false;
        }
    };
    if n == 0 {
        return false;
    }
    let mut y = 0;
    let mut x = 0;
    let mut flag = 0;
    loop {
        if matrix[y][x] == target {
            return true;
        } else if matrix[y][x] > target {
            break;
        } else {
            if x == n - 1 && y == m - 1 {
                return false;
            }
            if y != m - 1 {
                y += 1;
            } else {
                flag = 1;
            }
            if x != n - 1 {
                x += 1;
            } else {
                flag = 2;
            }
        }
    }
    if flag != 2 {
        for i in (0..=x).rev() {
            if matrix[y][i] == target {
                return true;
            } else if matrix[y][i] < target {
                for j in y + 1..m {
                    if matrix[j][i] > target {
                        break;
                    } else if matrix[j][i] == target {
                        return true;
                    }
                }
            }
        }
    }

    if flag != 1 {
        for i in (0..=y).rev() {
            if matrix[i][x] == target {
                return true;
            } else if matrix[i][x] < target {
                for j in x + 1..n {
                    if matrix[i][j] > target {
                        break;
                    } else if matrix[i][j] == target {
                        return true;
                    }
                }
            }
        }
    }
    false
}
