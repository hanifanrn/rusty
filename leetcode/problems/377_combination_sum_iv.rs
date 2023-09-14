struct Solution {}

impl Solution {
    pub fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
        let mut dp = vec![0; (target + 1) as usize];
        dp[0] = 1;

        for i in 1..=target as usize {
            for &num in &nums {
                if i as i32 - num >= 0 {
                    dp[i] += dp[(i as i32 - num) as usize];
                }
            }
        }

        return dp[target as usize];
    }
}

#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    fn main_test() {
        assert_eq!(Solution::combination_sum4(vec![1, 2, 3], 4), 7);
    }

    #[test]
    fn seccond_test() {
        assert_eq!(Solution::combination_sum4(vec![9], 3), 0);
    }
}
