use crate::my_solution::sixty;
use crate::my_solution::sixty::sixty::get_permutation;

#[test]
fn test_get_permutation() {
    assert_eq!(sixty::sixty::get_permutation(3, 2), String::from("132"));
    assert_eq!(sixty::sixty::get_permutation(3, 3), String::from("213"));
    assert_eq!(sixty::sixty::get_permutation(4, 9), String::from("2314"));
}

#[test]
fn test_rotate_right() {
    use sixty::sixty_one;
    use sixty_one::ListNode;
    let input = Some(Box::from(ListNode {
        val: 1,
        next: Some(Box::from(ListNode {
            val: 2,
            next: Some(Box::from(ListNode { val: 3, next: None })),
        })),
    }));
    let expect = Some(Box::from(ListNode {
        val: 3,
        next: Some(Box::from(ListNode {
            val: 1,
            next: Some(Box::from(ListNode { val: 2, next: None })),
        })),
    }));
    assert_eq!(sixty_one::rotate_right(input, 1), expect);
}

#[test]
fn test_unique_paths() {
    use sixty::sixty_two;
    assert_eq!(sixty_two::unique_paths(7, 3), 28);
}

#[test]
fn test_unique_paths_with_obstacles() {
    use sixty::sixty_three;
    assert_eq!(
        sixty_three::unique_paths_with_obstacles(vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]]),
        2
    );
    assert_eq!(
        sixty_three::unique_paths_with_obstacles(vec![
            vec![
                0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 1, 1, 0, 0, 1, 0, 1, 1, 0, 1, 0,
                0, 1, 0, 0, 0, 1, 0, 0
            ],
            vec![
                0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 1, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0
            ],
            vec![
                1, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 1,
                0, 1, 0, 0, 1, 0, 0, 0
            ],
            vec![
                1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 1, 0, 0, 0, 1, 0, 0, 0, 0,
                0, 0, 1, 0, 0, 1, 0, 1
            ],
            vec![
                0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 1, 0, 0
            ],
            vec![
                0, 0, 1, 0, 0, 0, 1, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 1,
                0, 0, 1, 0, 0, 1, 0, 0
            ],
            vec![
                0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 1, 0, 1, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 1, 0, 0, 1, 0, 0
            ],
            vec![
                1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0,
                1, 0, 1, 1, 0, 0, 0, 1
            ],
            vec![
                0, 0, 0, 0, 1, 0, 0, 1, 0, 1, 1, 1, 0, 0, 0, 1, 0, 0, 1, 0, 1, 0, 0, 0, 0, 1, 1, 1,
                0, 0, 0, 0, 0, 0, 0, 0
            ],
            vec![
                0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 1, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 1, 1, 0, 0, 0
            ],
            vec![
                1, 0, 1, 0, 1, 1, 0, 1, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 1, 1, 0, 0,
                0, 0, 1, 0, 0, 0, 1, 0
            ],
            vec![
                0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 1, 1, 0, 0, 0, 0, 0,
                1, 0, 0, 1, 0, 0, 0, 1
            ],
            vec![
                0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0,
                0, 1, 0, 0, 1, 0, 0, 0
            ],
            vec![
                1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0
            ],
            vec![
                0, 0, 1, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 1, 0
            ],
            vec![
                0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 1, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0,
                0, 1, 0, 0, 0, 0, 0, 0
            ],
            vec![
                0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 1, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0,
                1, 0, 0, 0, 0, 0, 0, 0
            ],
            vec![
                0, 0, 0, 0, 0, 1, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 1, 0
            ],
            vec![
                0, 0, 1, 0, 1, 0, 1, 0, 0, 0, 0, 0, 1, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0
            ],
            vec![
                0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 1, 0, 0, 0, 1, 0, 0, 0,
                1, 0, 0, 0, 1, 0, 0, 0
            ],
            vec![
                0, 0, 0, 1, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 1, 1, 1, 0, 1, 1, 1, 0, 0, 1, 0, 1, 0, 1,
                1, 1, 0, 0, 0, 0, 0, 0
            ],
            vec![
                0, 0, 1, 0, 0, 0, 0, 1, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0
            ],
            vec![
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 1, 0, 0, 0, 0, 0
            ],
            vec![
                0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0
            ],
            vec![
                1, 1, 0, 0, 0, 0, 1, 0, 0, 1, 1, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 1, 0
            ],
            vec![
                0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0,
                0, 0, 1, 0, 0, 1, 0, 0
            ],
            vec![
                0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0,
                1, 0, 0, 0, 1, 1, 0, 0
            ],
            vec![
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0,
                0, 0, 0, 0, 0, 0, 0, 0
            ]
        ]),
        718991952
    );
}

#[test]
fn test_min_path_sum() {
    use sixty::sixty_four;
    assert_eq!(
        sixty_four::min_path_sum(vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]]),
        7
    );
}

#[test]
fn test_is_number() {
    use sixty::sixty_five;
    assert!(sixty_five::is_number(String::from(" -90e3   ")));
    assert!(!sixty_five::is_number(String::from(" -+3  ")));
    assert!(!sixty_five::is_number(String::from("95a54e53   ")));
    assert!(sixty_five::is_number(String::from("  6e-1  ")));
    assert!(!sixty_five::is_number(String::from("  e  ")));
}

#[test]
fn test_plus_one() {
    use sixty::sixty_six;
    assert_eq!(sixty_six::plus_one(vec![1, 2, 3]), vec![1, 2, 4]);
    assert_eq!(sixty_six::plus_one(vec![9, 9]), vec![1, 0, 0]);
}
