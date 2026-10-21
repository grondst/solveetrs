// https://leetcode.com/problems/two-sum/
// Time: O(1) Space: O(?)

use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut end = HashMap::new();

        for (i, num) in nums.iter().enumerate() {
            let component = target - num;

            if let Some(&j) = end.get(&component) {
                return vec![j, i as i32];
            }

            end.insert(num, i as i32);
       }

        vec![]
    }
}

#[cfg(test)]
mod tests {
use super::*;

    #[test]
    fn case_1() {
        assert_eq!(Solution::two_sum(vec![2,7,11,15], 9), vec![0,1]);
    }

    fn case_2() {
        assert_eq!(Solution::two_sum(vec![3,2,4], 6), vec![1,2]);
    }

    fn case_3() {
        assert_eq!(Solution::two_sum(vec![3,3], 6), vec![0,1]);
    }
}
