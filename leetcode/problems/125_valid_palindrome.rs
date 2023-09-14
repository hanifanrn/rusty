struct Solution {}

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let s: String = s
            .chars()
            .filter(|c| c.is_alphanumeric())
            .collect::<String>()
            .to_lowercase();
        
        if s == s.chars().rev().collect::<String>() {
            return true;
        }

        return false;
    }
}

fn main() {
    let s = String::from("yehey");

    println!("{}", Solution::is_palindrome(s));
}

#[cfg(test)]
mod test{
    use crate::Solution;

    #[test]
    fn main_test() {
        assert_eq!(Solution::is_palindrome(String::from("hanifan")), false);
        assert_eq!(Solution::is_palindrome(String::from("aha aha")), true);
    }
}
