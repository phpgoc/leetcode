///给定一个 n x n 矩阵，其中每行和每列元素均按升序排序，找到矩阵中第 k 小的元素。
///请注意，它是排序后的第 k 小元素，而不是第 k 个不同的元素。
pub fn kth_smallest(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
    let mut flat = matrix.iter().flatten().collect::<Vec<_>>();
    flat.sort();
    *flat[k as usize - 1]
}
