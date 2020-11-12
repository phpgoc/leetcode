#[test]
fn length_of_lis() {
    use super::*;
    assert_eq!(length_of_lis(vec![10, 9, 2, 5, 3, 7, 101, 18]), 4);
    assert_eq!(
        use_greed::length_of_lis(vec![10, 9, 2, 5, 3, 7, 101, 18]),
        4
    );
}

#[test]
fn num_array() {
    use super::three;
    let model = three::NumArray::new(vec![-2, 0, 3, -5, 2, -1]);
    assert_eq!(model.sum_range(0, 2), 1);
    assert_eq!(model.sum_range(2, 5), -1);
    assert_eq!(model.sum_range(0, 5), -3);
}

#[test]
fn num_matrix() {
    use super::four;
    let model = four::NumMatrix::new(vec![
        vec![3, 0, 1, 4, 2],
        vec![5, 6, 3, 2, 1],
        vec![1, 2, 0, 1, 5],
        vec![4, 1, 0, 1, 7],
        vec![1, 0, 3, 0, 5],
    ]);
    assert_eq!(model.sum_region(2, 1, 4, 3), 8);
    assert_eq!(model.sum_region(1, 1, 2, 2), 11);
    assert_eq!(model.sum_region(1, 2, 2, 4), 12);
}
