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

#[test]
fn solution() {
    use super::four::Solution;
    let m = Solution::new(vec![1, 2, 3, 4]);
    let mut r = [0, 0, 0, 0];
    for _ in 0..1000 {
        let r1 = m.shuffle();
        for i in 0..4 {
            r[i] += r1[i];
        }
    }
    println!("{:?}", r);

    assert_eq!(m.reset(), vec![1, 2, 3, 4])
}

#[test]
fn deserialize() {
    use super::five;
    println!("{:?}", five::deserialize("[123,[456,[789]]]".to_string()));
}
#[test]
fn lexical_order() {
    use super::six;
    assert_eq!(
        six::lexical_order(13),
        vec![1, 10, 11, 12, 13, 2, 3, 4, 5, 6, 7, 8, 9]
    );
    six::lexical_order(5000000);
}

#[test]
fn first_uniq_char() {
    use super::seven;
    assert_eq!(seven::first_uniq_char(String::from("loveleetcode")), 2);
}

#[test]
fn find_the_difference() {
    use super::nine;
    assert_eq!(
        nine::find_the_difference(String::from("abcd"), String::from("abcde")),
        'e'
    );
}
