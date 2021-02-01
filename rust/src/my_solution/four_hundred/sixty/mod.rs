use std::cmp::Ordering;
use std::collections::{BTreeSet, HashMap};
///请你为 最不经常使用（LFU）缓存算法设计并实现数据结构。
//
// 实现 LFUCache 类：
//
// LFUCache(int capacity) - 用数据结构的容量 capacity 初始化对象
// int get(int key) - 如果键存在于缓存中，则获取键的值，否则返回 -1。
// void put(int key, int value) - 如果键已存在，则变更其值；如果键不存在，请插入键值对。当缓存达到其容量时，则应该在插入新项之前，使最不经常使用的项无效。在此问题中，当存在平局（即两个或更多个键具有相同使用频率）时，应该去除 最久未使用 的键。
// 注意「项的使用次数」就是自插入该项以来对其调用 get 和 put 函数的次数之和。使用次数会在对应项被移除后置为 0 。
//
// 为了确定最不常使用的键，可以为缓存中的每个键维护一个 使用计数器 。使用计数最小的键是最久未使用的键。
//
// 当一个键首次插入到缓存中时，它的使用计数器被设置为 1 (由于 put 操作)。对缓存中的键执行 get 或 put 操作，使用计数器的值将会递增。
//
// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/lfu-cache
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
struct LFUCache {
    data: HashMap<i32, Node>,
    capacity: usize,
    primary_key: i32,
    order_set: BTreeSet<Node>,
}
#[derive(Clone, Copy, Eq, PartialEq)]
struct Node {
    frequency: i32,
    id: i32,
    key: i32,
    val: i32,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.frequency.cmp(&other.frequency) {
            Ordering::Equal => self.id.cmp(&other.id),
            _o => _o,
        }
    }
}
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Node {
    fn new(id: i32, key: i32, val: i32) -> Self {
        Self {
            frequency: 1,
            id,
            key,
            val,
        }
    }
}

impl LFUCache {
    pub fn new(capacity: i32) -> Self {
        Self {
            data: Default::default(),
            capacity: capacity as usize,
            primary_key: 0,
            order_set: Default::default(),
        }
    }

    pub fn get(&mut self, key: i32) -> i32 {
        if self.capacity == 0 {
            return -1;
        }
        match self.data.get_mut(&key) {
            Some(t) => {
                self.order_set.remove(t);
                self.primary_key += 1;
                t.frequency += 1;
                t.id = self.primary_key;
                self.order_set.insert(*t);
                t.val
            }
            None => -1,
        }
    }

    pub fn put(&mut self, key: i32, value: i32) {
        if self.capacity == 0 {
            return;
        }
        match self.data.get_mut(&key) {
            Some(t) => {
                self.order_set.remove(t);
                self.primary_key += 1;
                t.frequency += 1;
                t.id = self.primary_key;
                t.val = value;
                self.order_set.insert(*t);
            }
            None => {
                self.primary_key += 1;
                if self.capacity == self.data.len() {
                    let for_remove_node = *self.order_set.iter().next().unwrap();
                    self.data.remove(&for_remove_node.key);
                    self.order_set.remove(&for_remove_node);
                }
                let node = Node::new(self.primary_key, key, value);
                self.order_set.insert(node);
                self.data.insert(key, node);
            }
        }
    }
}
pub mod eight;
pub mod four;
pub mod one;
pub mod seven;
#[cfg(test)]
mod tests;
pub mod three;
pub mod two;
