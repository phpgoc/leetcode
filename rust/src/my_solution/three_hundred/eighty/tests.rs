#[test]
fn randomized_set() {
    use super::*;
    let mut m = RandomizedSet::new();
    m.insert(1);
    assert!(m.insert(2));
    assert!(!m.insert(2));
    println!("{:?}", m.get_random());
}
#[test]
fn randomized_collection() {
    use super::one::RandomizedCollection;
    let mut m = RandomizedCollection::new();
    m.insert(1);
    assert!(m.insert(2));
    assert!(!m.insert(2));
    println!("{:?}", m.get_random());
}

#[test]
fn can_construct() {
    use super::three;
    assert!(three::can_construct("aa".to_string(), "aab".to_string()));
    assert!(!three::can_construct("a".to_string(), "b".to_string()));
    assert!(!three::can_construct(
        "fihjjjjei".to_string(),
        "hjibagacbhadfaefdjaeaebgi".to_string()
    ));
}
