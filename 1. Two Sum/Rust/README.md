# Question

Suppose you have an array of integers `nums` and a target integer `target`. Your task is to find two distinct integers in `nums` that add up to `target`. If such a pair exists, return their indices in the array as a tuple of size two. Otherwise, return `None`.

Write a function `two_sum(nums: &[i32], target: i32) -> Option<(usize, usize)>` in Rust that solves this problem. Your implementation should have a time complexity of O(n), where n is the length of the input slice `nums`. You are not allowed to use the same element twice.

For example, given the slice `nums = [2, 7, 11, 15]` and the target `target = 9`, the function should return `Some((0, 1))`, since `nums[0] + nums[1] = 2 + 7 = 9`.

Here's a template for the function signature:

```rs
fn two_sum(nums: &[i32], target: i32) -> Option<(usize, usize)> {
    unimplemented!()
}
```

## Follow-up questions

- Can you solve the problem with a space complexity of O(1)? How?
- What is the worst-case time complexity of your solution? Can you improve it further?
- Can you generalize your solution to find `k` distinct integers in `nums` that add up to `target`?
