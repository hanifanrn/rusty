pub struct Solution {}

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut low = 0;
        let mut high = nums.len() as i32 - 1;
        
        while low <= high {
            let middle = low + (high - low) / 2;    
            
            if nums[middle as usize] < target {
                low = middle + 1;
            } else if nums[middle as usize] == target {
                return middle;
            } else {
                high = middle - 1; 
            } 
        }

        return -1;  
    }
}

fn main() {

}

#[cfg(test)]
mod tests {
   use super::*; 

    #[test]
    fn it_works() {
        assert_eq!(Solution::search(vec![-3, -2, -1, 0, 1, 2, 3, 4, 5, 6, 7], 4), 7);        //target is 4
        assert_eq!(Solution::search(vec![0, 1, 2], 7), -1);    //target is 7 
        assert_eq!(Solution::search(vec![0, 2, 4, 6, 8], 0), 4);    //target is 0 this test should not pass
        assert_eq!(2 + 2, 5); //this test should not pass 
    }
}
