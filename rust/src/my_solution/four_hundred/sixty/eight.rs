///编写一个函数来验证输入的字符串是否是有效的 IPv4 或 IPv6 地址。
//
// 如果是有效的 IPv4 地址，返回 "IPv4" ；
// 如果是有效的 IPv6 地址，返回 "IPv6" ；
// 如果不是上述类型的 IP 地址，返回 "Neither" 。
// IPv4 地址由十进制数和点来表示，每个地址包含 4 个十进制数，其范围为 0 - 255， 用(".")分割。比如，172.16.254.1；
//
// 同时，IPv4 地址内的数不会以 0 开头。比如，地址 172.16.254.01 是不合法的。
//
// IPv6 地址由 8 组 16 进制的数字来表示，每组表示 16 比特。这些组数字通过 (":")分割。比如,  2001:0db8:85a3:0000:0000:8a2e:0370:7334 是一个有效的地址。而且，我们可以加入一些以 0 开头的数字，字母可以使用大写，也可以是小写。所以， 2001:db8:85a3:0:0:8A2E:0370:7334 也是一个有效的 IPv6 address地址 (即，忽略 0 开头，忽略大小写)。
//
// 然而，我们不能因为某个组的值为 0，而使用一个空的组，以至于出现 (::) 的情况。 比如， 2001:0db8:85a3::8A2E:0370:7334 是无效的 IPv6 地址。
//
// 同时，在 IPv6 地址中，多余的 0 也是不被允许的。比如， 02001:0db8:85a3:0000:0000:8a2e:0370:7334 是无效的。
//
// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/validate-ip-address
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
pub fn valid_ip_address(ip: String) -> String {
    let split_by_dot = ip.split('.').collect::<Vec<_>>();
    match split_by_dot.len() {
        4 => {
            if valid_ipv4(split_by_dot) {
                return String::from("IPv4");
            } else {
                return String::from("Neither");
            }
        }
        1 => {}
        _ => {
            return String::from("Neither");
        }
    }

    let split_by_colon = ip.split(':').collect::<Vec<_>>();
    match split_by_colon.len() {
        8 => {
            if valid_ipv6(split_by_colon) {
                return String::from("IPv6");
            } else {
                return String::from("Neither");
            }
        }
        _ => {
            return String::from("Neither");
        }
    }
}
fn valid_ipv4(vec: Vec<&str>) -> bool {
    for i in vec {
        if i.len() == 0 || i.len() != 1 && i.starts_with('0') {
            dbg!(i);
            return false;
        }
        match i.parse::<i32>() {
            Ok(t) => {
                if t > 255 {
                    return false;
                }
            }
            Err(_) => {
                return false;
            }
        }
    }
    true
}
const CHAR_IN_16: [char; 22] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'A', 'B', 'C',
    'D', 'E', 'F',
];
fn valid_ipv6(vec: Vec<&str>) -> bool {
    for i in vec {
        if i.len() == 0 || i.len() > 4 {
            return false;
        }
        for j in i.chars() {
            if !CHAR_IN_16.contains(&j) {
                return false;
            }
        }
    }
    true
}
