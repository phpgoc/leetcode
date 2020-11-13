///累加数是一个字符串，组成它的数字可以形成累加序列。
//
// 一个有效的累加序列必须至少包含 3 个数。除了最开始的两个数以外，字符串中的其他数都等于它之前两个数相加的和。
//
// 给定一个只包含数字 '0'-'9' 的字符串，编写一个算法来判断给定输入是否是累加数。
//
// 说明: 累加序列里的数不会以 0 开头，所以不会出现 1, 2, 03 或者 1, 02, 3 的情况。
//
// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/additive-number
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
pub fn is_additive_number(num: String) -> bool {
    let mut stack = vec![];
    for i in 1..=num.len() / 2 {
        stack.push(*&num[0..i].parse::<i64>().unwrap());
        for j in 1..=(num.len() - i) / 2 {
            stack.push(*&num[i..i + j].parse::<i64>().unwrap());
            if dfs(i.max(j), i + j, &mut stack, &num) {
                return true;
            }
            stack.pop();
            if num[i..].chars().next().unwrap() == '0' {
                break;
            }
        }

        stack.pop();
    }
    false
}

fn dfs(str_len: usize, from: usize, stack: &mut Vec<i64>, num: &String) -> bool {
    // println!(
    //     "stack = {:?} from = {},num.len() = {},str_len ={}",
    //     stack,
    //     from,
    //     num.len(),
    //     str_len
    // );

    if from == num.len() {
        return true;
    }
    if num.len() - from < str_len {
        return false;
    }
    if num[from..].chars().next().unwrap() == '0'
        && stack[stack.len() - 1] != 0
        && stack[stack.len() - 2] != 0
    {
        return false;
    }
    let mut this_num = *&num[from..from + str_len].parse::<i64>().unwrap();

    // println!("this_num = {:?}", this_num);

    if this_num == stack[stack.len() - 1] + stack[stack.len() - 2] {
        stack.push(this_num);
        if dfs(str_len, from + str_len, stack, num) {
            return true;
        }
        stack.pop();
    }

    if num.len() - from >= str_len + 1 {
        this_num = *&num[from..from + str_len + 1].parse::<i64>().unwrap();
        if this_num == stack[stack.len() - 1] + stack[stack.len() - 2] {
            stack.push(this_num);
            if dfs(str_len + 1, from + str_len + 1, stack, num) {
                return true;
            }
            stack.pop();
        }
    }
    false
}
