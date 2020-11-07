#[test]
fn single_number() {
    use super::single_number;
    let result = single_number(vec![1, 2, 1, 3, 2, 5]);
    let expect = vec![3, 5];
    assert_eq!(result.len(), expect.len());
    for i in expect {
        assert!(result.contains(&i));
    }
}

#[test]
fn is_ugly() {
    use super::three;
    assert!(three::is_ugly(6));
    assert!(!three::is_ugly(17));
}

#[test]
fn nth_ugly_number() {
    use super::four;
    assert_eq!(four::nth_ugly_number(10), 12);
    println!("{:?}", four::nth_ugly_number(1690));
}
