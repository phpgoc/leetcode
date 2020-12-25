///给定一个二维的甲板， 请计算其中有多少艘战舰。 战舰用 'X'表示，空位用 '.'表示。 你需要遵守以下规则：
//
// 给你一个有效的甲板，仅由战舰或者空位组成。
// 战舰只能水平或者垂直放置。换句话说,战舰只能由 1xN (1 行, N 列)组成，或者 Nx1 (N 行, 1 列)组成，其中N可以是任意大小。
// 两艘战舰之间至少有一个水平或垂直的空位分隔 - 即没有相邻的战舰。
//
// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/battleships-in-a-board
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
pub fn count_battleships(board: Vec<Vec<char>>) -> i32 {
    let mut res = 0;
    let (x_len, y_len) = match board.first() {
        Some(t) => ((*t).len(), board.len()),
        _ => return res,
    };
    if x_len == 0 {
        return res;
    }
    for y in 0..y_len {
        for x in 0..x_len {
            if board[y][x] == 'X' {
                res += match (y, x) {
                    (0, 0) => 1,
                    (0, _) => {
                        if board[y][x - 1] == 'X' {
                            0
                        } else {
                            1
                        }
                    }
                    (_, 0) => {
                        if board[y - 1][x] == 'X' {
                            0
                        } else {
                            1
                        }
                    }
                    (_, _) => {
                        if board[y][x - 1] == 'X' || board[y - 1][x] == 'X' {
                            0
                        } else {
                            1
                        }
                    }
                };
            }
        }
    }
    res
}
