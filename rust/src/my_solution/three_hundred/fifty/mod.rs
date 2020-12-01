pub mod five;
pub mod four;
#[cfg(test)]
mod tests;
pub mod two;
///给定两个数组，编写一个函数来计算它们的交集。
pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let mut res = vec![];
    let mut nums1 = nums1;
    let mut nums2 = nums2;
    nums1.sort();
    nums2.sort();
    let mut i1 = 0;
    let mut i2 = 0;
    while i1 < nums1.len() && i2 < nums2.len() {
        while i1 < nums1.len() && i2 < nums2.len() && nums1[i1] != nums2[i2] {
            if nums1[i1] < nums2[i2] {
                i1 += 1;
            } else {
                i2 += 1;
            }
        }
        if i1 < nums1.len() && i2 < nums2.len() {
            res.push(nums1[i1]);
            i1 += 1;
            i2 += 1;
        }
    }
    res
}
