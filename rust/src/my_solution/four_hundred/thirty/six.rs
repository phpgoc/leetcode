use std::collections::BTreeMap;

///给定一组区间，对于每一个区间 i，检查是否存在一个区间 j，它的起始点大于或等于区间 i 的终点，这可以称为 j 在 i 的“右侧”。
///
///对于任何区间，你需要存储的满足条件的区间 j 的最小索引，这意味着区间 j 有最小的起始点可以使其成为“右侧”区间。如果区间 j 不存在，则将区间 i 存储为 -1。最后，你需要输出一个值为存储的区间值的数组。
pub fn find_right_interval(intervals: Vec<Vec<i32>>) -> Vec<i32> {
    let mut map = BTreeMap::new();
    for (k, v) in intervals.iter().enumerate() {
        map.entry(v[0]).or_insert(k);
    }
    let mut res = vec![];
    for i in intervals.iter() {
        res.push(match map.range(i[1]..).next() {
            Some((_, &t)) => t as i32,
            None => -1,
        });
    }
    res
}
