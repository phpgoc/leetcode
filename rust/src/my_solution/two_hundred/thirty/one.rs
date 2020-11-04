///给定一个整数，编写一个函数来判断它是否是 2 的幂次方。
pub fn is_power_of_two(n: i32) -> bool {
    let mut n = n;
    let mut left = 0;
    loop{
        left = n&7;
        n>>=3;
        if n ==0{
            return left ==1|| left ==2 || left ==4
        }else if  left != 0{
            return false;
        }
    }
}