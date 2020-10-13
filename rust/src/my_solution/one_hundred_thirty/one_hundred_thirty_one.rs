pub fn partition(s: String) -> Vec<Vec<String>> {
    let chars = s.chars().collect::<Vec<_>>();
    let len = chars.len();
    let mut res = vec![];
    let mut dp = vec![vec![false; len]; len];
    for i in 0..len {
        dp[i][i] = true;
    }
    for i in 1..len {
        for j in 0..len - i {
            if chars[j] == chars[j + i] && (i == 1 || dp[j + 1][j + i - 1] == true) {
                dp[j][j + i] = true;
            }
        }
    }
    //    println!("{:?}", dp);
    let mut vec = vec![];
    for i in 0..len {
        if dp[0][i] == true {
            vec.clear();
            dfs(0, i, &mut vec, &chars, &mut res, &dp, &len);
        }
    }
    //    println!("{:?}", res);

    res
}

fn dfs(
    i: usize,
    j: usize,
    vec: &mut Vec<String>,
    chars: &Vec<char>,
    res: &mut Vec<Vec<String>>,
    dp: &Vec<Vec<bool>>,
    len: &usize,
) {
    let mut str = String::new();
    for k in i..=j {
        str.push(chars[k]);
    }
    //    println!("str = {:?},i = {}, j = {}", str, i, j);

    vec.push(str);
    if j == len - 1 {
        res.push((*vec).clone());
        return;
    }
    for k in j + 1..*len {
        if dp[j + 1][k] == true {
            dfs(j + 1, k, vec, chars, res, dp, len);
            vec.pop();
        }
    }
}
