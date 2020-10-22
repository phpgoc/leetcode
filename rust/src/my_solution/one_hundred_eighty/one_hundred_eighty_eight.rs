use std::collections::HashMap;

pub fn max_profit(k: i32, prices: Vec<i32>) -> i32 {
    let len = prices.len();
    let mut map = HashMap::new();
    if len / 2 <= k as usize {
        return max_profit_unlimited(prices);
    } else {
        dfs((0, 0, 0), &k, &len, &prices, &mut map, 0)
    }
}
fn max_profit_unlimited(prices: Vec<i32>) -> i32 {
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

fn dfs(
    params: (usize, i32, i32),
    k: &i32,
    len: &usize,
    prices: &Vec<i32>,
    map: &mut HashMap<(usize, i32, i32), i32>,
    buy_price: i32,
) -> i32 {
    match map.get(&params) {
        Some(&t) => {
            return t;
        }
        _ => {}
    }
    if (params.0 == *len || params.2 == *k) {
        return 0;
    }
    //保持不动
    let mut res = dfs(
        (params.0 + 1, params.1, params.2),
        k,
        len,
        prices,
        map,
        buy_price,
    );
    if params.1 == 1 {
        //卖一股，并将交易次数+1
        if buy_price < prices[params.0] {
            res = res.max(
                dfs((params.0 + 1, 0, params.2 + 1), k, len, prices, map, 0) + prices[params.0],
            );
        }
    } else {
        res = res.max(
            dfs(
                (params.0 + 1, 1, params.2),
                k,
                len,
                prices,
                map,
                prices[params.0],
            ) - prices[params.0],
        );
    }
    map.insert(params, res);
    res
}
