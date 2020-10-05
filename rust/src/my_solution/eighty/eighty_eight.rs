pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    let mut m = m as usize;
    let mut n = n as usize;
    while n != 0 {
        if m == 0 {
            for i in (0..n).rev() {
                nums1[i] = nums2[i];
            }
            break;
        }
        if nums1[m - 1] < nums2[n - 1] {
            nums1[m + n - 1] = nums2[n - 1];
            n -= 1;
        } else {
            nums1[m + n - 1] = nums1[m - 1];
            m -= 1;
        }
    }
}
