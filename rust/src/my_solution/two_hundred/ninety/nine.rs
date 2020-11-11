use std::cmp::Ordering;
///你在和朋友一起玩 猜数字（Bulls and Cows）游戏，该游戏规则如下：
//
// 你写出一个秘密数字，并请朋友猜这个数字是多少。
// 朋友每猜测一次，你就会给他一个提示，告诉他的猜测数字中有多少位属于数字和确切位置都猜对了（称为“Bulls”, 公牛），有多少位属于数字猜对了但是位置不对（称为“Cows”, 奶牛）。
// 朋友根据提示继续猜，直到猜出秘密数字。
// 请写出一个根据秘密数字和朋友的猜测数返回提示的函数，返回字符串的格式为 xAyB ，x 和 y 都是数字，A 表示公牛，用 B 表示奶牛。
//
// xA 表示有 x 位数字出现在秘密数字中，且位置都与秘密数字一致。
// yB 表示有 y 位数字出现在秘密数字中，但位置与秘密数字不一致。
// 请注意秘密数字和朋友的猜测数都可能含有重复数字，每位数字只能统计一次。
//
// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/bulls-and-cows
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
pub fn get_hint(secret: String, guess: String) -> String {
    let mut a = 0;
    let mut b = 0;
    let mut secret_vec = secret.chars().collect::<Vec<_>>();
    let mut guess_vec = guess.chars().collect::<Vec<_>>();
    let len = secret_vec.len();
    for i in 0..len {
        if secret_vec[i] == guess_vec[i] {
            a += 1;
        }
    }
    secret_vec.sort();
    guess_vec.sort();
    let mut s = 0;
    let mut q = 0;
    while s != len && q != len {
        match secret_vec[s].cmp(&(guess_vec[q])) {
            Ordering::Less => {
                s += 1;
            }
            Ordering::Equal => {
                b += 1;
                s += 1;
                q += 1;
            }
            Ordering::Greater => {
                q += 1;
            }
        }
    }
    format!("{}A{}B", a, b - a)
}
