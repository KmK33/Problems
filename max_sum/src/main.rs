fn max_subarray_sum(nums: &[i32]) -> i32 {
    if nums.is_empty() {
        return 0;
    }

    let mut max_current = nums[0];
    let mut max_global = nums[0];

    for &num in &nums[1..] {
        max_current = std::cmp::max(num, max_current + num);
        if max_current > max_global {
            max_global = max_current;
        }
    }

    max_global
}

fn main() {
    let nums = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
    println!("Max subarray sum : {}", max_subarray_sum(&nums));  
}

