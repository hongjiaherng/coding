pub struct Solution;

impl Solution {
    /// # Proof: Floyd's Tortoise and Hare Algorithm
    ///
    /// ## Setup
    /// We interpret `nums` as a linked structure: from index `i` you move to `nums[i]`.
    /// Because `nums` has `n+1` values all in `1..=n`, there must be a cycle
    /// (by the pigeonhole principle).
    ///
    /// Define:
    /// - `μ` (mu): distance from the start (index 0) to the entry of the cycle
    /// - `λ` (lambda): length of the cycle
    /// - `x`: distance from the the intersection point back to the entry point of the cycle
    ///
    /// ```text
    /// start ─── μ steps ───► cycle entry ──  (λ - x) steps ──► intersection
    ///                          ▲                                   │
    ///                          └──────────────── x ────────────────┘
    /// ```
    ///
    /// ## Phase 1: meeting inside the cycle
    /// Let the tortoise (slow) move one step at a time and the hare (fast) two steps.
    /// Suppose they meet after `t` steps of the tortoise -- the intersection point:
    ///
    /// - Slow has traveled:
    ///   ```text
    ///   distance_slow = t = μ + (λ - x)
    ///   ```
    /// - Fast has traveled:
    ///   ```text
    ///   distance_fast = 2t = μ + (λ - x) + kλ
    ///   ```
    ///   for some integer `k ≥ 1`, since the hare may loop around the cycle multiple times.
    ///
    /// Now subtract the two equations:
    /// ```text
    /// distance_fast - distance_slow = (μ + (λ - x) + kλ) - (μ + (λ - x))
    /// 2t - t = kλ
    /// => t = kλ
    /// ```
    ///
    /// This tells us the tortoise has traveled a multiple of the cycle length `λ`
    /// by the time it meets the hare.
    /// Equivalently, the intersection point is `(λ - x)` steps past the cycle entry,
    /// because `t = μ + (λ - x)`.
    ///
    /// ## Phase 2: finding the entry
    /// From above we know:
    /// ```text
    /// t = μ + (λ - x)
    /// and t = kλ
    /// => μ + (λ - x) ≡ 0 (mod λ)
    /// => μ ≡ x (mod λ)
    /// ```
    ///
    /// This statement shows:
    /// - If one pointer starts from the beginning (distance `μ` away),
    /// - And the other starts from the intersection (distance `x` away),
    /// - Moving both forward step by step will cause them to meet exactly at
    ///   the cycle entry after `μ` steps.
    ///
    /// ## Conclusion
    /// Resetting one pointer to the start and advancing both at the same speed
    /// ensures they meet at the cycle entry.
    /// The cycle entry corresponds to the duplicate number in the array, so the
    /// algorithm correctly returns the duplicate.
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        // linear time
        // constant extra space, no hashmap
        // contains n + 1 integers, each integer in range [1,n]

        // Find the intersection
        let mut slow = nums[0];
        let mut fast = nums[0];
        loop {
            slow = nums[slow as usize];
            fast = nums[nums[fast as usize] as usize]; // Move 2 steps
            if slow == fast {
                break;
            }
        }

        // Reset one pointer to the start and advance both at the same speed to find the cycle entry
        slow = nums[0];
        while slow != fast {
            slow = nums[slow as usize];
            fast = nums[fast as usize];
        }

        slow
    }
}

#[cfg(test)]
mod tests {
    use crate::linked_list::q287_find_the_duplicate_number::Solution;

    // Input: nums = [1,3,4,2,2]
    // Output: 2
    #[test]
    fn test_example_1() {
        // [0, 3, 1, 2]
        let nums = vec![1, 3, 4, 2, 2];
        assert_eq!(Solution::find_duplicate(nums), 2);
    }

    // Input: nums = [3,1,3,4,2]
    // Output: 3
    #[test]
    fn test_example_2() {
        let nums = vec![3, 1, 3, 4, 2];
        assert_eq!(Solution::find_duplicate(nums), 3);
    }

    // Input: nums = [3,3,3,3,3]
    // Output: 3
    #[test]
    fn test_example_3() {
        let nums = vec![3, 3, 3, 3, 3];
        assert_eq!(Solution::find_duplicate(nums), 3);
    }

    #[test]
    fn test_example_4() {
        let nums = vec![2, 5, 9, 6, 9, 3, 8, 9, 7, 1];
        assert_eq!(Solution::find_duplicate(nums), 9);
    }
}
