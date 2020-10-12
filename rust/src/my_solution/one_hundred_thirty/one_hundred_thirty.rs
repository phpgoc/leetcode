use std::borrow::Borrow;
use std::collections::HashSet;

pub fn solve(board: &mut Vec<Vec<char>>) {
    let y = board.len();
    if y == 0 {
        return;
    }
    let x = board[0].len();
    let mut set = HashSet::new();
    for i in 0..x {
        if board[0][i] == 'O' {
            dfs(&board, &mut set, i, 0, &x, &y);
        }
        if board[y - 1][i] == 'O' {
            dfs(&board, &mut set, i, y - 1, &x, &y);
        }
    }
    for i in 1..y - 1 {
        if board[i][0] == 'O' {
            dfs(&board, &mut set, 0, i, &x, &y);
        }
        if board[i][x - 1] == 'O' {
            dfs(&board, &mut set, x - 1, i, &x, &y);
        }
    }
    for y_i in 0..y {
        for x_i in 0..x {
            if !set.contains((y_i, x_i).borrow()) {
                board[y_i][x_i] = 'X';
            }
        }
    }
}

fn dfs(
    board: &Vec<Vec<char>>,
    set: &mut HashSet<(usize, usize)>,
    x: usize,
    y: usize,
    x_len: &usize,
    y_len: &usize,
) {
    set.insert((y, x));
    if x != 0 {
        if board[y][x - 1] == 'O' && !set.contains((y, x - 1).borrow()) {
            dfs(board, set, x - 1, y, x_len, y_len);
        }
    }
    if y != 0 {
        if board[y - 1][x] == 'O' && !set.contains((y - 1, x).borrow()) {
            dfs(board, set, x, y - 1, x_len, y_len);
        }
    }
    if x != x_len - 1 {
        if board[y][x + 1] == 'O' && !set.contains((y, x + 1).borrow()) {
            dfs(board, set, x + 1, y, x_len, y_len);
        }
    }
    if y != y_len - 1 {
        if board[y + 1][x] == 'O' && !set.contains((y + 1, x).borrow()) {
            dfs(board, set, x, y + 1, x_len, y_len);
        }
    }
}
