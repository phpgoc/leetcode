#[test]
fn randomized_set() {
    use super::*;
    let mut m = RandomizedSet::new();
    m.insert(1);
    assert!(m.insert(2));
    assert!(!m.insert(2));
    println!("{:?}", m.get_random());
}
