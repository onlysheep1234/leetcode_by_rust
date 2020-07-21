/**
给你数字 k ，请你返回和为 k 的斐波那契数字的最少数目，其中，每个斐波那契数字都可以被使用多次。

斐波那契数字定义为：

    F1 = 1
    F2 = 1
    Fn = Fn-1 + Fn-2 ， 其中 n > 2 。

数据保证对于给定的 k ，一定能找到可行解。

 

示例 1：

输入：k = 7
输出：2 
解释：斐波那契数字为：1，1，2，3，5，8，13，……
对于 k = 7 ，我们可以得到 2 + 5 = 7 。

示例 2：

输入：k = 10
输出：2 
解释：对于 k = 10 ，我们可以得到 2 + 8 = 10 。

示例 3：

输入：k = 19
输出：3 
解释：对于 k = 19 ，我们可以得到 1 + 5 + 13 = 19 。

 

提示：

    1 <= k <= 10^9

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/find-the-minimum-number-of-fibonacci-numbers-whose-sum-is-k
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
 * **/

pub struct Solution{}

#[allow(dead_code)]
impl Solution {
    pub fn find_min_fibonacci_numbers(k: i32) -> i32 {
        if k == 1 {
            return 1;
        }

        if k == 2 {
            return 2;
        }

        let mut l = vec![1,1];
        loop {
            let last = l.last().unwrap() + l[l.len()-2];
            if last > k {
                break;
            }

            l.push(last);
        }

        let mut k = k;
        let mut count = 0;
        for i in l.into_iter().rev() {
            if i == k {
                return count+1;
            } else if i < k {
                k = k-i;
                count+=1; 
            }
        }

        return count;
    }
}


#[cfg(test)]
mod tests{
    #[test]
    fn test_find_min_fib_bumbers() {
        use super::Solution;
        assert_eq!(Solution::find_min_fibonacci_numbers(7), 2);
    }
}