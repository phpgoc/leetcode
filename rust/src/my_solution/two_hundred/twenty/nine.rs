use counter::Counter;

///给定一个大小为 n 的整数数组，找出其中所有出现超过 ⌊ n/3 ⌋ 次的元素。
//
// 进阶：尝试设计时间复杂度为 O(n)、空间复杂度为 O(1)的算法解决此问题。
//
// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/majority-element-ii
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
    let k = 2;
    let len = nums.len();
    let mut res = vec![];
    if len == 0{return res;}
    let mut cands_cnts = vec![(nums[0],0);k];

    //第1阶段 成对抵销
    'outer :for &i in &nums{
        for j in 0..k{
            if cands_cnts[j].0 == i{
                cands_cnts[j].1 +=1;
                continue 'outer;
            }
        }
        let mut flag = true;
        for j in 0..k{
            if cands_cnts[j].1 == 0{
                cands_cnts[j] = (i,1);
                flag = false;
                break;
            }
        }
        if flag{
            for j in 0..k{
                cands_cnts[j].1 -=1;
            }
        }
    }
    for j in 0..k{
        cands_cnts[j].1 = 0;
    }

    for &i in &nums {
        for j in 0..k{
            if i==cands_cnts[j].0{
                cands_cnts[j].1 +=1;
                break;
            }
        }
    }
    // println!("cands_cnts = {:?}",cands_cnts);
    
    for i in cands_cnts{
        if i.1>len/(k+1){
            res.push(i.0);
        }
    }
    res.sort();
    res
}
///这个方法就是用来验证上边的结果是否正确
pub fn majority_element_use_counter(nums: Vec<i32>) -> Vec<i32> {
    let k = 2;
    let counter = nums.iter().collect::<Counter<_>>();
    // println!("most_common = {:?}",counter.most_common());
    let mut res = vec![];
    for i in counter.most_common(){
        if i.1 > nums.len()/(k+1){
            res.push(*i.0);
        }
    }
    res.sort();
    res
}
