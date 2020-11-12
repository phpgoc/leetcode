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
