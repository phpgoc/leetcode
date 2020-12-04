use std::collections::HashMap;
extern "C" {
    fn srand() -> u32;
    fn rand() -> u32;
}
///设计一个支持在平均 时间复杂度 O(1) 下， 执行以下操作的数据结构。
//
// 注意: 允许出现重复元素。
//
// insert(val)：向集合中插入元素 val。
// remove(val)：当 val 存在时，从集合中移除一个 val。
// getRandom：从现有集合中随机获取一个元素。每个元素被返回的概率应该与其在集合中的数量呈线性相关。
//
// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/insert-delete-getrandom-o1-duplicates-allowed
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
pub struct RandomizedCollection {
    data: HashMap<i32, i32>,
    list: Vec<i32>,
    len: usize,
    capacity: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RandomizedCollection {
    /** Initialize your data structure here. */
    pub fn new() -> Self {
        Self {
            data: Default::default(),
            list: vec![],
            len: 0,
            capacity: 0,
        }
    }

    /** Inserts a value to the collection. Returns true if the collection did not already contain the specified element. */
    pub fn insert(&mut self, val: i32) -> bool {
        let v = self.data.entry(val).or_insert(0);
        *v += 1;
        if self.capacity == self.len {
            self.list.push(val);
            self.capacity += 1;
        } else {
            self.list[self.len] = val;
        }
        self.len += 1;
        return *v == 1;
    }

    /** Removes a value from the collection. Returns true if the collection contained the specified element. */
    pub fn remove(&mut self, val: i32) -> bool {
        if let Some(v) = self.data.get_mut(&val) {
            if *v == 0 {
                return false;
            }
            *v -= 1;
            let remove_index = self.list.iter().position(|r| r == &val).unwrap();

            self.list.swap(remove_index, self.len - 1);
            self.len -= 1;
            return true;
        } else {
            return false;
        }
    }

    /** Get a random element from the collection. */
    pub fn get_random(&self) -> i32 {
        let random = unsafe { rand() } as usize % self.len;
        self.list[random]
    }
}
