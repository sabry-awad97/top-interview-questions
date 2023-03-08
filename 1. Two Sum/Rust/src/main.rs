use std::collections::HashMap;

fn two_sum(nums: &[i32], target: i32) -> Option<(usize, usize)> {
    // Create a new HashMap to store previously seen elements and their indices
    let mut complements = HashMap::new();

    // Loop through each element of the input array and its index
    for (i, &num) in nums.iter().enumerate() {
        // Check if the target minus the current element is in the HashMap
        if let Some(&j) = complements.get(&(target - num)) {
            // If it is, then return a tuple containing the indices of the two elements
            return Some((j, i));
        }
        // If the target minus the current element is not in the HashMap,
        // add the current element and its index to the HashMap for future reference
        complements.insert(num, i);
    }

    // If no solution is found, return None
    None
}

fn main() {
    let nums = vec![1, 2, 3, 4];
    let target = 5;
    let result = two_sum(&nums, target);
    println!("{:?}", result); // Outputs Some((1, 2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum() {
        // Test case 1: Basic example
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        let expected = Some((0, 1));
        let result = two_sum(&nums, target);
        assert_eq!(result, expected);

        // Test case 2: No solution exists
        let nums = vec![2, 7, 11, 15];
        let target = 4;
        let expected = None;
        let result = two_sum(&nums, target);
        assert_eq!(result, expected);
    }
}
