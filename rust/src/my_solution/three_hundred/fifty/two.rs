use std::collections::BTreeMap;

///给定一个非负整数的数据流输入 a1，a2，…，an，…，将到目前为止看到的数字总结为不相交的区间列表。
pub struct SummaryRanges {
    data: BTreeMap<i32, i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SummaryRanges {
    /** Initialize your data structure here. */
    pub fn new() -> Self {
        Self {
            data: BTreeMap::new(),
        }
    }

    pub fn add_num(&mut self, val: i32) {
        if let Some(_) = self.data.get(&val) {
            return;
        }
        let mut from = val;
        let mut to = val;
        match self.data.range(..val).next_back() {
            Some((&k, &v)) => {
                if v >= val {
                    return;
                } else if v == val - 1 {
                    from = k;
                }
            }
            None => {}
        };
        match self.data.range(val + 1..).next() {
            Some((&k, &v)) => {
                if k == val + 1 {
                    self.data.remove(&(val + 1));
                    to = v;
                }
            }
            None => {}
        };
        self.data.insert(from, to);
    }

    pub fn get_intervals(&self) -> Vec<Vec<i32>> {
        let mut res = vec![];
        for (&k, &v) in self.data.iter() {
            res.push(vec![k, v]);
        }
        res
    }
}
