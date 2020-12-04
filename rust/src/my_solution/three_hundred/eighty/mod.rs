use std::collections::HashSet;
extern "C" {
    fn srand() -> u32;
    fn rand() -> u32;
}
///380. 常数时间插入、删除和获取随机元素
// 设计一个支持在平均 时间复杂度 O(1) 下，执行以下操作的数据结构。
//
// insert(val)：当元素 val 不存在时，向集合中插入该项。
// remove(val)：元素 val 存在时，从集合中移除该项。
// getRandom：随机返回现有集合中的一项。每个元素应该有相同的概率被返回。
pub struct RandomizedSet {
    data: HashSet<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RandomizedSet {
    /** Initialize your data structure here. */
    pub fn new() -> Self {
        Self {
            data: Default::default(),
        }
    }

    /** Inserts a value to the set. Returns true if the set did not already contain the specified element. */
    fn insert(&mut self, val: i32) -> bool {
        self.data.insert(val)
    }

    /** Removes a value from the set. Returns true if the set contained the specified element. */
    fn remove(&mut self, val: i32) -> bool {
        self.data.remove(&val)
    }

    /** Get a random element from the set. */
    fn get_random(&self) -> i32 {
        let random = unsafe {
            srand();
            rand()
        } as usize
            % self.data.len();
        *self.data.iter().nth(random).unwrap()
    }
}
pub mod one;
#[cfg(test)]
mod tests;
