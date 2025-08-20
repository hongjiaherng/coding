use std::cmp::Ordering;

pub struct Solution;

impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let mut left = 1;
        let mut right = *piles.iter().max().unwrap();
        let mut min_k = right;

        while left <= right {
            let k = left + (right - left) / 2;

            let mut hours_left = h;
            for pile in &piles {
                hours_left -= (pile + k - 1) / k;
                if hours_left < 0 {
                    break; // already too slow
                }
            }

            match hours_left.cmp(&0) {
                Ordering::Greater | Ordering::Equal => {
                    // eat too fast, make it eat slower
                    min_k = min_k.min(k);
                    right = k - 1;
                }
                Ordering::Less => {
                    // eat too slow, make it eat faster
                    left = k + 1;
                }
            }
        }
        min_k
    }
}

#[cfg(test)]
mod tests {
    use crate::binary_search::q875_koko_eating_bananas::Solution;

    #[test]
    fn test_min_eating_speed() {
        let piles = vec![3, 6, 7, 11];
        let h = 8;
        assert_eq!(Solution::min_eating_speed(piles, h), 4);
    }

    #[test]
    fn test_min_eating_speed_example_2() {
        let piles = vec![30, 11, 23, 4, 20];
        let h = 5;
        assert_eq!(Solution::min_eating_speed(piles, h), 30);
    }

    #[test]
    fn test_min_eating_speed_example_3() {
        let piles = vec![30, 11, 23, 4, 20];
        let h = 6;
        assert_eq!(Solution::min_eating_speed(piles, h), 23);
    }

    #[test]
    fn test_min_eating_speed_example_4() {
        let piles = vec![312884470];
        let h = 312884469;
        assert_eq!(Solution::min_eating_speed(piles, h), 2);
    }

    #[test]
    fn test_min_eating_speed_example_5() {
        let piles = vec![1000000000, 1000000000];
        let h = 3;
        assert_eq!(Solution::min_eating_speed(piles, h), 1000000000);
    }

    #[test]
    fn test_min_eating_speed_example_6() {
        let piles = vec![805306368, 805306368, 805306368];
        let h = 1000000000;
        assert_eq!(Solution::min_eating_speed(piles, h), 3);
    }
}
