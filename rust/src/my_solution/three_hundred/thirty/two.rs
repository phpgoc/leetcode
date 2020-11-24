use std::collections::HashMap;
///给定一个机票的字符串二维数组 [from, to]，子数组中的两个成员分别表示飞机出发和降落的机场地点，对该行程进行重新规划排序。所有这些机票都属于一个从 JFK（肯尼迪国际机场）出发的先生，所以该行程必须从 JFK 开始。

pub fn find_itinerary(tickets: Vec<Vec<String>>) -> Vec<String> {
    let mut res = vec![];
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    for i in tickets.iter() {
        match map.get_mut(&i[0]) {
            Some(t) => {
                t.push(i[1].clone());
            }
            None => {
                map.insert(i[0].clone(), vec![i[1].clone()]);
            }
        }
    }
    for i in map.iter_mut() {
        i.1.sort_by(|a, b| b.cmp(a));
    }

    let mut cur = String::from("JFK");
    let mut from = cur.clone();
    let mut stack = vec![];
    stack.push(cur.clone());
    while !stack.is_empty() {
        loop {
            if let Some(m_v) = map.get_mut(&cur) {
                if let Some(t) = m_v.pop() {
                    cur = t;
                    stack.push(cur.clone());

                    if cur == from {
                        break;
                    }
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        while let Some(top) = stack.pop() {
            if let Some(t) = map.get(&top) {
                if !t.is_empty() {
                    from = top;
                    cur = from.clone();
                    stack.push(cur.clone());
                    break;
                } else {
                    res.push(top.clone());
                }
            } else {
                res.push(top.clone());
            }
        }
    }
    res.reverse();
    res
}
