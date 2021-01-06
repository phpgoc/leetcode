#[test]
fn find_kth_number() {
    assert_eq!(super::find_kth_number(13, 2), 10);
}
#[test]
fn arrange_coins() {
    use super::one;
    assert_eq!(one::arrange_coins(8), 3);
}
#[test]
fn find_duplicates() {
    use super::two;
    let res = two::find_duplicates(vec![4, 3, 2, 7, 8, 2, 3, 1]);
    let expect = vec![2, 3];
    assert_eq!(res.len(), expect.len());
    for i in res {
        assert!(expect.contains(&i));
    }
    let res = two::find_duplicates(vec![
        13, 8, 5, 3, 20, 12, 3, 20, 5, 16, 9, 19, 12, 11, 13, 19, 11, 1, 10, 2,
    ]);
    let expect = vec![3, 20, 5, 12, 13, 19, 11];
    assert_eq!(res.len(), expect.len());
    for i in res {
        assert!(expect.contains(&i));
    }
}
