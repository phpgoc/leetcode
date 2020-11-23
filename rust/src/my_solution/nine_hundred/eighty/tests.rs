#[test]
fn unique_paths_iii() {
    assert_eq!(
        super::unique_paths_iii(vec![vec![1, 0, 0, 0], vec![0, 0, 0, 0], vec![0, 0, 2, -1]]),
        2
    );
}
