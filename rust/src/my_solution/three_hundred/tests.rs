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

#[test]
fn is_additive_number() {
    use super::six;
    assert!(six::is_additive_number(String::from("112358")));
    assert!(six::is_additive_number(String::from("199100199")));
    assert!(six::is_additive_number(String::from("000")));
    assert!(!six::is_additive_number(String::from("199001200")));
}

#[test]
fn num_array2() {
    use super::seven;
    let mut model = seven::NumArray::new(vec![-2, 0, 3, -5, 2, -1]);
    assert_eq!(model.sum_range(0, 2), 1);
    assert_eq!(model.sum_range(2, 5), -1);
    assert_eq!(model.sum_range(0, 5), -3);
    model.update(0, 1);
    assert_eq!(model.sum_range(0, 5), 0);
}
