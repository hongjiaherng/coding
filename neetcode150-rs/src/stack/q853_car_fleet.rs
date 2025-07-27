pub struct Solution;

impl Solution {
    pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
        let pairs: Vec<(i32, i32)> = {
            let mut v: Vec<(i32, i32)> = position.into_iter().zip(speed).collect();
            v.sort_by(|a, b| b.0.cmp(&a.0));
            v
        };

        let mut stack: Vec<f64> = vec![];
        for (p, s) in pairs {
            // time to reach target
            let ttr = (target - p) as f64 / s as f64;
            if stack.is_empty() || &ttr > stack.last().unwrap() {
                stack.push(ttr);
            }
        }

        stack.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::stack::q853_car_fleet::Solution;

    /// Example 1:
    /// Input: target = 12, position = [10,8,0,5,3], speed = [2,4,1,1,3]
    /// Output: 3
    /// Explanation:
    /// The cars starting at 10 (speed 2) and 8 (speed 4) become a fleet, meeting each other at 12. The fleet forms at target.
    /// The car starting at 0 (speed 1) does not catch up to any other car, so it is a fleet by itself.
    /// The cars starting at 5 (speed 1) and 3 (speed 3) become a fleet, meeting each other at 6. The fleet moves at speed 1 until it reaches target.
    #[test]
    fn example1() {
        assert_eq!(
            Solution::car_fleet(12, vec![10, 8, 0, 5, 3], vec![2, 4, 1, 1, 3]),
            3
        );
    }

    /// Example 2:
    /// Input: target = 10, position = [3], speed = [3]
    /// Output: 1
    /// Explanation:
    /// There is only one car, hence there is only one fleet.
    #[test]
    fn example2() {
        assert_eq!(Solution::car_fleet(10, vec![3], vec![3]), 1);
    }

    /// Example 3:
    /// Input: target = 100, position = [0,2,4], speed = [4,2,1]
    /// Output: 1
    /// Explanation:
    /// The cars starting at 0 (speed 4) and 2 (speed 2) become a fleet, meeting each other at 4. The car starting at 4 (speed 1) travels to 5.
    /// Then, the fleet at 4 (speed 2) and the car at position 5 (speed 1) become one fleet, meeting each other at 6. The fleet moves at speed 1 until it reaches target.
    #[test]
    fn example3() {
        assert_eq!(Solution::car_fleet(100, vec![0, 2, 4], vec![4, 2, 1]), 1);
    }

    /// Edge Case: No cars
    /// Input: target = 10, position = [], speed = []
    /// Output: 0
    /// Explanation:
    /// No cars, so no fleets.
    #[test]
    fn no_cars() {
        assert_eq!(Solution::car_fleet(10, vec![], vec![]), 0);
    }

    #[test]
    fn example4() {
        assert_eq!(
            Solution::car_fleet(10, vec![8, 3, 7, 4, 6, 5], vec![4, 4, 4, 4, 4, 4]),
            6
        );
    }
}
