struct Solution {}

impl Solution {
    pub fn min_deletions(s: String) -> i32 {
        let mut freq = [0; 26];
        for b in s.bytes() {
            freq[(b - b'a') as usize] += 1;
        }
        freq.sort_unstable();

        let mut ans = 0;
        for i in (0..freq.len() - 1).rev() {
            let diff = if freq[i + 1] == 0 {
                freq[i]
            } else if freq[i + 1] <= freq[i] {
                freq[i] - freq[i + 1] + 1
            } else {
                0
            };
            ans += diff;
            freq[i] -= diff;
        }

        return ans;
    }
}


#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    fn main_test() {
        assert_eq!(Solution::min_deletions(String::from("aab")), 0);
    }

    #[test]
    fn seccond_test() {
        assert_eq!(Solution::min_deletions(String::from("aaabbbcc")), 2)
    }
   
    #[test]
    fn third_test() {
        assert_eq!(Solution::min_deletions(String::from("ceabaacb")), 2)
    }
}
