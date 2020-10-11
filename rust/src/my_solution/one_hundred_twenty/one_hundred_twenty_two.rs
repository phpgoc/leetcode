pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut buy = prices[0];
    let mut sell = 0;
    let mut res = 0;
    for i in prices {
        if i < sell {
            if sell > buy {
                res += sell - buy;
            }
            buy = i;
            sell = i;
        } else {
            sell = i;
        }
        //        println!("i= {},buy = {},sell = {} ,res = {}", i, buy, sell, res);
    }
    if sell > buy {
        res += sell - buy;
    }
    res
}
