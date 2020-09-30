pub fn total_n_queens(n: i32) -> i32 {
    if n == 1 {
        return 1;
    }
    let mut answer = 0;
    let mut option = vec![];
    for x in 0..n {
        option.clear();
        option.push(x);
        dfs(n, &mut option, 1, &mut answer);
    }
    answer
}

fn dfs(n: i32, option: &mut Vec<i32>, y: i32, answer: &mut i32) {
    'outer: for i in 0..n {
        for j in 0..option.len() {
            if (y - j as i32) == (i - option[j]).abs() || option[j] == i {
                continue 'outer;
            }
        }
        option.push(i);
        if y == n - 1 {
            *answer += 1;
            option.pop();
            return;
        }
        dfs(n, option, y + 1, answer);
        option.pop();
    }
}
