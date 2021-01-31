#[test]
fn lfucache() {
    use super::LFUCache;
    let mut model = LFUCache::new(2);
    model.put(2, 3);
    assert_eq!(model.get(2), 3);
    model.put(3, 4);
    model.put(4, 5);
    assert_eq!(model.get(3), -1);
}
#[test]
fn hamming_distance() {
    use super::one;
    assert_eq!(one::hamming_distance(1, 4), 2);
    assert_eq!(one::hamming_distance(1, 5), 1);
}
#[test]
fn min_moves2() {
    use super::two;
    assert_eq!(two::min_moves2(vec![1, 2, 3]), 2);
    assert_eq!(two::min_moves2(vec![1, 2, 2, 3]), 2);
    assert_eq!(two::min_moves2(vec![1, 0, 0, 8, 6]), 14);
}
#[test]
fn island_perimeter() {
    use super::three;
    let grid = [[0, 1, 0, 0], [1, 1, 1, 0], [0, 1, 0, 0], [1, 1, 0, 0]]
        .iter()
        .map(|r| r.to_vec())
        .collect::<Vec<_>>();
    assert_eq!(three::island_perimeter(grid), 16);
}
#[test]
fn can_i_win() {
    use super::four;
    assert!(!four::can_i_win(10, 11));
}
