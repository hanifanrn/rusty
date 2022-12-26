use::std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::with_capacity(nums.len());
        
        for (index, val) in nums.iter().enumerate() {

            match map.get(&(target - val)) {
                None => {
                    map.insert(val, index);
                }
                Some(prev_index) => {
                    return vec![*prev_index as i32, index as i32]
                }
            }
        }
        vec![]
    }    
}