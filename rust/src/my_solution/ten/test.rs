use crate::my_solution::ten;
#[test]
fn test_two_sum() {
    let nums = vec![2, 3, 5, 7];
    let targat = 9;
    let answer = ten::one::two_sum(nums, targat);
    assert_eq!(answer, vec![0, 3])
}

#[test]
fn test_add_two_numbers() {
    use ten::two;

    let l1 = Some(Box::new(two::ListNode {
        val: 2,
        next: Some(Box::new(two::ListNode {
            val: 4,
            next: Some(Box::new(two::ListNode { val: 3, next: None })),
        })),
    }));
    let l2 = Some(Box::new(two::ListNode {
        val: 5,
        next: Some(Box::new(two::ListNode {
            val: 6,
            next: Some(Box::new(two::ListNode { val: 4, next: None })),
        })),
    }));
    let result = Some(Box::new(two::ListNode {
        val: 7,
        next: Some(Box::new(two::ListNode {
            val: 0,
            next: Some(Box::new(two::ListNode { val: 8, next: None })),
        })),
    }));
    assert_eq!(two::add_two_numbers(l1, l2), result);
}
#[test]
fn test_length_of_longest_substring() {
    use ten::three;
    assert_eq!(
        three::length_of_longest_substring(String::from("pwwkew")),
        3
    );
    assert_eq!(three::length_of_longest_substring(String::from("bbbbb")), 1);
    assert_eq!(
        three::length_of_longest_substring(String::from("abcabcbb")),
        3
    );
}
#[test]
fn test_longest_palindrome() {
    use ten::four;
    assert_eq!(
        four::longest_palindrome(String::from("cabaderta")),
        String::from("aba")
    );
    assert_eq!(
        four::longest_palindrome(String::from("aa")),
        String::from("aa")
    );
}

#[test]
fn test_convert() {
    use ten::six;
    assert_eq!(
        six::convert(String::from("LEETCODEISHIRING"), 3),
        String::from("LCIRETOESIIGEDHN")
    );
}

#[test]
fn test_reverse() {
    use ten::seven;
    assert_eq!(seven::reverse(123), 321);
}
