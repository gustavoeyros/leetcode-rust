use std::collections::HashMap;

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut num_index_map = HashMap::new();

    for (index, num) in nums.iter().enumerate() {
        let diff = target - num;
        if let Some(&diff_index) = num_index_map.get(&diff) {
            return vec![index as i32, diff_index as i32];
        }
        num_index_map.insert(num, index);
    }
    Vec::new()
}

fn main() {
    let result = two_sum(vec![3, 2, 4], 6);
    println!("{:?}", result);
}
