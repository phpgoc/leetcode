#[test]
fn min_patches() {
    assert_eq!(super::min_patches(vec![1, 3], 6), 1);
    assert_eq!(super::min_patches(vec![1, 2, 31, 33], 2147483647), 28);
}
