use std::collections::{HashMap, VecDeque};

pub struct Solution {}
impl Solution {
    pub fn water_puzzle(buckets: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut res = vec![];
        let len = buckets.len();
        let mut map = HashMap::new();
        let zero = vec![0; len];
        map.insert(zero.clone(), zero.clone());
        let mut queue = VecDeque::new();
        queue.push_back(zero.clone());
        let mut end = vec![];
        't: while let Some(cur) = queue.pop_front() {
            // println!("t = {:?}",cur);

            let mut nexts = vec![];
            for i in 0..len {
                let mut cur_clone = cur.clone();
                cur_clone[i] = buckets[i];
                nexts.push(cur_clone);
                cur_clone = cur.clone();
                cur_clone[i] = 0;
                nexts.push(cur_clone);
                for j in 0..len {
                    if i == j {
                        continue;
                    }
                    cur_clone = cur.clone();
                    cur_clone[i] -= cur[i].min(buckets[j] - cur[j]);
                    cur_clone[j] += cur[i].min(buckets[j] - cur[j]);
                    nexts.push(cur_clone);
                }
            }
            // println!("nexts = {:?}",nexts);

            for next in nexts {
                if map.contains_key(&next) {
                    continue;
                }
                map.insert(next.clone(), cur.clone());
                queue.push_back(next.clone());
                if next.contains(&target) {
                    end = next;
                    break 't;
                }
            }
            // println!("map = {:?}",map);
        }
        if !end.is_empty() {
            let mut add_to_res = end;
            while add_to_res != zero {
                res.push(add_to_res.clone());
                add_to_res = map.get(&*add_to_res).unwrap().to_vec();
            }
            res.push(zero);
            res.reverse();
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_water_puzzle() {
        println!("{:?}", Solution::water_puzzle(vec![3, 5], 4));
        println!("{:?}", Solution::water_puzzle(vec![100, 31, 17], 63));
    }
}
