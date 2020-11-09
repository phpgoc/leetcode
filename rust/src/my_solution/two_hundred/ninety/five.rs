use std::cmp::Reverse;
use std::collections::BinaryHeap;

///中位数是有序列表中间的数。如果列表长度是偶数，中位数则是中间两个数的平均值。
//
// 例如，
//
// [2,3,4] 的中位数是 3
//
// [2,3] 的中位数是 (2 + 3) / 2 = 2.5
//
// 设计一个支持以下两种操作的数据结构：
//
// void addNum(int num) - 从数据流中添加一个整数到数据结构中。
// double findMedian() - 返回目前所有元素的中位数。
//
// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/find-median-from-data-stream
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
#[derive(Debug)]
pub struct MedianFinder {
    big: BinaryHeap<Reverse<i32>>,
    small: BinaryHeap<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MedianFinder {
    /** initialize your data structure here. */
    pub fn new() -> Self {
        MedianFinder {
            small: BinaryHeap::new(),
            big: BinaryHeap::new(),
        }
    }

    pub fn add_num(&mut self, num: i32) {
        if self.big.len() > self.small.len() {
            self.big.push(Reverse(num));
            self.small.push(self.big.pop().unwrap().0);
        } else {
            self.small.push(num);
            self.big.push(Reverse(self.small.pop().unwrap()));
        }
        // println!("{:?}", self);
    }

    pub fn find_median(&self) -> f64 {
        if self.big.len() > self.small.len() {
            self.big.peek().unwrap().0 as f64
        } else {
            *self.small.peek().unwrap() as f64
                + (self.big.peek().unwrap().0 - *self.small.peek().unwrap()) as f64 / 2.0
        }
    }
}
pub mod use_btree_map {
    use std::collections::BTreeMap;

    #[derive(Debug)]
    pub struct MedianFinder {
        low: (i32, i32),
        high: (i32, i32),
        data: BTreeMap<i32, i32>,
        len: i32,
    }
    impl MedianFinder {
        /** initialize your data structure here. */
        pub fn new() -> Self {
            MedianFinder {
                low: (std::i32::MIN, 0),
                high: (std::i32::MAX, 1),
                data: BTreeMap::new(),
                len: 0,
            }
        }

        pub fn add_num(&mut self, num: i32) {
            match self.data.get_mut(&num) {
                Some(t) => {
                    *t += 1;
                }
                None => {
                    self.data.insert(num, 1);
                }
            }
            self.len += 1;

            let merge = self.len % 2 == 1;
            if num > self.high.0 {
                if merge {
                    self.low = self.high;
                } else {
                    if self.high.1 != *self.data.get(&self.high.0).unwrap() {
                        self.high.1 += 1;
                    } else {
                        let (&a, &_) = self.data.range(self.high.0 + 1..).next().unwrap();
                        self.high = (a, 1);
                    }
                }
            } else if num == self.high.0 {
                self.low = self.high;
                if !merge {
                    self.high.1 += 1;
                }
            } else if num < self.low.0 {
                if merge {
                    self.high = self.low;
                } else {
                    if self.low.1 != 1 {
                        self.low.1 -= 1;
                    } else {
                        let (&a, &b) = self.data.range(..self.low.0).next_back().unwrap();
                        self.low = (a, b);
                    }
                }
            } else if num == self.low.0 {
                self.low.1 += 1;
                self.high = self.low;
            // 不合并的和 num == self.low.0 一样 ，不会进入这个分支
            } else {
                if merge {
                    self.low = (num, 1);
                    self.high = (num, 1);
                } else {
                    if self.low.0 > num {
                        self.low = (num, 1);
                    } else {
                        self.high = (num, 1);
                    }
                }
            }
            // println!("num = {},data = {:?}", num, self);
        }

        pub fn find_median(&self) -> f64 {
            (self.low.0 + self.high.0) as f64 / 2.0
        }
    }
}
