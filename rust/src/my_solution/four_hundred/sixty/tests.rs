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
