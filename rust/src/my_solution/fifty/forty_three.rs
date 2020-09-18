use std::char;
use std::collections::VecDeque;

pub fn multiply(num1: String, num2: String) -> String {
    if "0" == num1 || "0" == num2 {
        return String::from("0");
    }
    let mut answer = VecDeque::new();

    let num1_len = num1.len();
    let num2_len = num2.len();
    let mut num1_vec: Vec<_> = num1.chars().collect();
    num1_vec.reverse();
    let mut num2_vec: Vec<_> = num2.chars().collect();
    num2_vec.reverse();
    let mut i = 0;
    let mut left = 0;
    while num1_len + num2_len > i + 1 {
        for i1 in 0..num1_len {
            if i1 + num2_len <= i || i < i1 {
                continue;
            }
            left += (num1_vec[i1].to_string().parse::<u32>().unwrap())
                * (num2_vec[i - i1].to_string().parse::<u32>().unwrap());
        }
        let to_add: char = char::from_digit((left % 10), 10).unwrap();
        left = left / 10;
        answer.push_front(to_add);
        i += 1;
    }
    if left != 0 {
        answer.push_front(char::from_digit(left, 10).unwrap())
    }
    let mut answer_string = String::new();
    for i in answer {
        answer_string.push(i);
    }
    answer_string
}
