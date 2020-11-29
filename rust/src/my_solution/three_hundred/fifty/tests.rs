#[test]
fn intersect() {
    let mut res = super::intersect(vec![1, 2, 2, 1], vec![2, 2]);
    res.sort();
    assert_eq!(res, [2, 2]);
}
