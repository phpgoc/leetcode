pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
    if n == 1 {
        return vec![vec![String::from("Q")]];
    }
    let mut answer = vec![];
    let mut option = vec![];
    for x in 0..n {
        option.clear();
        option.push(x);
        dfs(n, &mut option, 1, &mut answer);
    }
    answer
}

fn dfs(n: i32, option: &mut Vec<i32>, y: i32, answer: &mut Vec<Vec<String>>) {
    'outer: for i in 0..n {
        for j in 0..option.len() {
            if (y - j as i32) == (i - option[j]).abs() || option[j] == i {
                continue 'outer;
            }
        }
        option.push(i);
        if y == n - 1 {
            println!("{:?} ,y = {}", option, y);

            let mut one_option_for_answer = vec![];
            for add_i in 0..n as usize {
                let mut str = String::new();
                for add_j in 0..n {
                    if add_j == option[add_i] {
                        str.push('Q');
                    } else {
                        str.push('.');
                    }
                }
                one_option_for_answer.push(str);
            }
            answer.push(one_option_for_answer);
            option.pop();
            return;
        }
        dfs(n, option, y + 1, answer);
        option.pop();
    }
}
