use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub struct MinStack {
    data: Vec<i32>,
    min: BinaryHeap<Reverse<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {
    /** initialize your data structure here. */
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            min: BinaryHeap::new(),
        }
    }

    pub fn push(&mut self, x: i32) {
        self.data.push(x);
        self.min.push(Reverse(x));
    }

    pub fn pop(&mut self) {
        let e = self.data.pop();
        if let Some(t) = e {
            let Reverse(min_v) = self.min.peek().unwrap();
            if *min_v == t {
                self.min.pop();
            }
        }
        println!("{:?}", self.data);
    }

    pub fn top(&self) -> i32 {
        return match self.data.last() {
            Some(t) => *t,
            _ => 0,
        };
    }

    pub fn get_min(&self) -> i32 {
        return match self.min.peek() {
            Some(Reverse(t)) => *t,
            _ => 0,
        };
    }
}
