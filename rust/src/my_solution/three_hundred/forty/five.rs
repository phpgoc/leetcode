///编写一个函数，以字符串作为输入，反转该字符串中的元音字母。

pub fn reverse_vowels(s: String) -> String {
    let mut chars = s.chars().collect::<Vec<_>>();
    if chars.len() == 0 {
        return s;
    }
    let vowels = ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];
    let mut left = 0;
    let mut right = chars.len() - 1;
    loop {
        while left < right && !vowels.contains(&chars[left]) {
            left += 1;
        }
        while left < right && !vowels.contains(&chars[right]) {
            right -= 1;
        }
        if left < right {
            chars.swap(left, right);
            left += 1;
            right -= 1;
        } else {
            break;
        }
    }
    chars.iter().collect::<String>()
}
