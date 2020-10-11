pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut res = 0;
    let mut min_value = std::i32::MAX;
    for i in prices {
        min_value = min_value.min(i);
        res = res.max(i - min_value);
    }
    res
}
