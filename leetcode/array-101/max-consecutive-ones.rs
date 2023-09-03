struct Solution {}

impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let mut local_max:i32 = 0;
        let mut max:i32 = 0;
        
        for i in nums {
            if i == 1 {
                local_max += 1;
            } else {
                if local_max > max {
                    max = local_max;
                }

                local_max = 0;
            }
        }

        if local_max > max {
            max = local_max;
        }
        
        return max;
    }
}

fn main() {
    println!("{:?}", Solution::find_max_consecutive_ones(vec![1,1,0,1,1,1]));
}

#[cfg(test)]
mod test{
    use crate::Solution;

    #[test]
    fn main_test() {
        assert_eq!(Solution::find_max_consecutive_ones(vec![1,1,0,1,1,1]), 3);
        assert_eq!(Solution::find_max_consecutive_ones(vec![1,0,1,1,0,1]), 2);
    }
}

