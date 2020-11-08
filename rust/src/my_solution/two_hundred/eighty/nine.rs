use lazy_static::lazy_static;
lazy_static! {
    static ref DIR: Vec<Vec<(i32, i32)>> = {
        vec![
            vec![
                (-1, -1),
                (-1, 0),
                (-1, 1),
                (0, -1),
                (0, 1),
                (1, -1),
                (1, 0),
                (1, 1),
            ], // middle
            vec![(0, 1), (1, 0), (1, 1)], // left up 1
            vec![(0, -1), (1, 0), (1, -1)], // right up 2
            vec![(0, 1), (-1, 0), (-1, 1)], // left down 3
            vec![(0, -1), (-1, 0), (-1, -1)], // right down 4
            vec![(-1, 0), (-1, 1), (0, 1), (1, 0), (1, 1)], //left 5
            vec![(-1, 0), (-1, -1), (0, -1), (1, 0), (1, -1)], //right 6
            vec![(0, -1), (0, 1), (1, -1), (1, 0), (1, 1)], //up 7
            vec![(0, -1), (0, 1), (-1, -1), (-1, 0), (-1, 1)], //down 8
            vec![(0,1)], //left 9
            vec![(0,-1)], // right 10
            vec![(0,-1),(0,1)], // one row middle 11
            vec![(1,0)], //up 12
            vec![(-1,0)], // down  13
            vec![(1,0),(-1,0)], // one column middle 14
        ]
    };
}
///根据 百度百科 ，生命游戏，简称为生命，是英国数学家约翰·何顿·康威在 1970 年发明的细胞自动机。
//
// 给定一个包含 m × n 个格子的面板，每一个格子都可以看成是一个细胞。每个细胞都具有一个初始状态：1 即为活细胞（live），或 0 即为死细胞（dead）。每个细胞与其八个相邻位置（水平，垂直，对角线）的细胞都遵循以下四条生存定律：
//
// 如果活细胞周围八个位置的活细胞数少于两个，则该位置活细胞死亡；
// 如果活细胞周围八个位置有两个或三个活细胞，则该位置活细胞仍然存活；
// 如果活细胞周围八个位置有超过三个活细胞，则该位置活细胞死亡；
// 如果死细胞周围正好有三个活细胞，则该位置死细胞复活；
// 根据当前状态，写一个函数来计算面板上所有细胞的下一个（一次更新后的）状态。下一个状态是通过将上述规则同时应用于当前状态下的每个细胞所形成的，其中细胞的出生和死亡是同时发生的。
//
// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/game-of-life
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
    let (y_len, x_len) = match board.first() {
        Some(t) => (board.len(), t.len()),
        _ => {
            return;
        }
    };
    if x_len == 0 {
        return;
    }
    let mut dp = vec![vec![(0, 0, 0); x_len]; y_len];
    if x_len == 1 && y_len == 1 {
        board[0][0] = 0;
        return;
    } else if y_len == 1 {
        dp[0][0].2 = 9;
        dp[0][x_len - 1].2 = 10;
        for i in 1..x_len - 1 {
            dp[0][i].2 = 11;
        }
    } else if x_len == 1 {
        dp[0][0].2 = 12;
        dp[y_len - 1][0].2 = 13;
        for i in 1..y_len - 1 {
            dp[i][0].2 = 14;
        }
    } else {
        dp[0][0].2 = 1;
        dp[0][x_len - 1].2 = 2;
        dp[y_len - 1][0].2 = 3;
        dp[y_len - 1][x_len - 1].2 = 4;
        for i in 1..y_len - 1 {
            dp[i][0].2 = 5;
            dp[i][x_len - 1].2 = 6;
        }
        for i in 1..x_len - 1 {
            dp[0][i].2 = 7;
            dp[y_len - 1][i].2 = 8;
        }
    }

    for y in 0..y_len {
        for x in 0..x_len {
            if board[y][x] == 1 {
                for &i in &(*DIR)[dp[y][x].2] {
                    dp[(y as i32 + i.0) as usize][(x as i32 + i.1) as usize].0 += 1;
                }
            }
        }
    }
    // let mut undone = true;
    // loop {
    //     undone = true;
    for y in 0..y_len {
        for x in 0..x_len {
            if board[y][x] == 1 {
                if dp[y][x].0 < 2 || dp[y][x].0 > 3 {
                    for &i in &(*DIR)[dp[y][x].2] {
                        dp[(y as i32 + i.0) as usize][(x as i32 + i.1) as usize].1 -= 1;
                    }

                    board[y][x] = 0;
                    // undone = false;
                }
            } else {
                if dp[y][x].0 == 3 {
                    for &i in &(*DIR)[dp[y][x].2] {
                        dp[(y as i32 + i.0) as usize][(x as i32 + i.1) as usize].1 += 1;
                        board[y][x] = 1;
                    }
                    // undone = false;
                }
            }
        }
    }
    // println!("dp --------------------------------");
    // for i in &dp {
    //     println!("{:?}", i);
    // }
    // println!("board **************************");
    // for i in &*board {
    //     println!("{:?}", i);
    // }
    // if undone {
    //     break;
    // } else {
    //     for y in 0..y_len {
    //         for x in 0..x_len {
    //             dp[y][x].0 += dp[y][x].1;
    //             dp[y][x].1 = 0;
    //         }
    //     }
    // }
    // }
}
