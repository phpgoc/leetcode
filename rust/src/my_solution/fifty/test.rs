use crate::my_solution::fifty;

#[test]
fn test_first_missing_positive() {
    use fifty::forty_one;
    assert_eq!(forty_one::first_missing_positive(vec![3, 4, -1, 1]), 2);
}

#[test]
fn test_trap() {
    use fifty::forty_two;
    assert_eq!(forty_two::trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]), 6);
    assert_eq!(forty_two::trap(vec![4, 2, 3]), 1);
    assert_eq!(forty_two::trap(vec![4, 2, 0, 3, 2, 4, 3, 4]), 10);
    assert_eq!(forty_two::trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]), 6);
}

#[test]
fn test_multiply() {
    use fifty::forty_three;
    assert_eq!(
        forty_three::multiply(String::from("3"), String::from("2")),
        String::from("6")
    );
    assert_eq!(
        forty_three::multiply(String::from("123"), String::from("456")),
        String::from("56088")
    );
}

#[test]
fn test_jump() {
    use fifty::forty_five;
    assert_eq!(forty_five::jump(vec![2, 3, 1, 1, 4]), 2);
}

#[test]
fn test_permute() {
    use fifty::forty_six;
    let expect = vec![
        vec![1, 2, 3],
        vec![1, 3, 2],
        vec![2, 1, 3],
        vec![2, 3, 1],
        vec![3, 1, 2],
        vec![3, 2, 1],
    ];
    let input = vec![1, 2, 3];
    let result = forty_six::permute(input);
    for i in expect {
        assert!(result.contains(&i));
    }
}

#[test]
fn test_permute_unique() {
    use fifty::forty_seven;
    let expect = vec![vec![1, 1, 2], vec![1, 2, 1], vec![2, 1, 1]];
    let input = vec![1, 2, 1];
    let result = forty_seven::permute_unique(input);
    println!("{:?}", result);
    for i in expect {
        assert!(result.contains(&i));
    }
}

#[test]
fn test_rotate() {
    use fifty::forty_eight;
    let mut input = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    forty_eight::rotate(&mut input);
    assert_eq!(input, vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3]])
}
