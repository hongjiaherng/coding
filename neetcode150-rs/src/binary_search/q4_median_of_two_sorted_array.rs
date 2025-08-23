pub struct Solution;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        if nums1.len() > nums2.len() {
            // always binary search for shorter array so we got log(min(m,n))
            return Self::find_median_sorted_arrays(nums2, nums1);
        }

        let (m, n) = (nums1.len() as i32, nums2.len() as i32);
        let (mut left, mut right) = (0, m);
        let half = (m + n + 1) / 2; // half include the middle point if total is odd

        while left <= right {
            let partition_a = left + (right - left) / 2; // size of left partition in array a
            let partition_b = half - partition_a; // size of left partition in array b

            let max_left_a = if partition_a - 1 >= 0 {
                // if left partition of a is not empty
                nums1[(partition_a - 1) as usize]
            } else {
                i32::MIN
            };
            let max_left_b = if partition_b - 1 >= 0 {
                // if left partition of b is not empty
                nums2[(partition_b - 1) as usize]
            } else {
                i32::MIN
            };
            let min_right_a = if partition_a < m {
                // if right partition of a is not empty
                nums1[partition_a as usize]
            } else {
                i32::MAX
            };
            let min_right_b = if partition_b < n {
                // if right partition of b is not empty
                nums2[partition_b as usize]
            } else {
                i32::MAX
            };

            if max_left_a <= min_right_b && max_left_b <= min_right_a {
                if (m + n) % 2 == 1 {
                    return max_left_a.max(max_left_b) as f64;
                }
                return (max_left_a.max(max_left_b) + min_right_a.min(min_right_b)) as f64 / 2f64;
            } else if max_left_a > min_right_b {
                right = partition_a - 1;
            } else {
                left = partition_a + 1;
            }
        }
        unreachable!();
    }
}

#[cfg(test)]
mod tests {
    use crate::binary_search::q4_median_of_two_sorted_array::Solution;

    #[test]
    fn test_find_median_sorted_arrays_example_1() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 3], vec![2]),
            2.0
        );
    }

    #[test]
    fn test_find_median_sorted_arrays_example_2() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]),
            2.5
        );
    }

    #[test]
    fn test_find_median_sorted_arrays_example_3() {
        assert_eq!(Solution::find_median_sorted_arrays(vec![], vec![1]), 1.0);
    }

    #[test]
    fn test_find_median_sorted_arrays_example_4() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![2], vec![1, 3]),
            2.0
        );
    }

    #[test]
    fn test_find_median_sorted_arrays_example_5() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 2, 3, 4], vec![1, 2, 3, 4, 5, 6, 7, 8]),
            3.5
        );
    }
}
