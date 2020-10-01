pub fn min_distance(word1: String, word2: String) -> i32 {
    let word1_vec: Vec<_> = word1.chars().collect();
    let word2_vec: Vec<_> = word2.chars().collect();
    let word1_len = word1_vec.len();
    let word2_len = word2_vec.len();
    if word1_len == 0 || word2_len == 0 {
        return word1_len.max(word2_len) as i32;
    }
    let mut dp = vec![vec![0; word2_len + 1]; word1_len + 1];

    for i in 0..=word1_len {
        dp[i][0] = i as i32;
    }
    for i in 1..=word2_len {
        dp[0][i] = i as i32;
    }
    let mut left_up = 0;
    for word1_i in 1..=word1_len {
        for word2_i in 1..=word2_len {
            if word1_vec[word1_i - 1] == word2_vec[word2_i - 1] {
                left_up = dp[word1_i - 1][word2_i - 1] - 1;
            } else {
                left_up = dp[word1_i - 1][word2_i - 1];
            }
            dp[word1_i][word2_i] =
                left_up.min(dp[word1_i][word2_i - 1].min(dp[word1_i - 1][word2_i])) + 1;
        }
    }
    //    for i in &dp {
    //        println!("{:?}", i);
    //    }

    dp[word1_len][word2_len]
}
