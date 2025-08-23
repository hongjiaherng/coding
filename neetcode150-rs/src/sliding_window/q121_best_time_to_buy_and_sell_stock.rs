use std::cmp::Ordering;
pub struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut max_profit = 0;
        let (mut l, mut r) = (0, 1);

        while r < prices.len() {
            match prices[l].cmp(&prices[r]) {
                Ordering::Less => {
                    let profit = prices[r] - prices[l];
                    max_profit = max_profit.max(profit);
                }
                Ordering::Equal | Ordering::Greater => l = r,
            }
            r += 1;
        }

        max_profit
    }
}

#[cfg(test)]
mod tests {
    use crate::sliding_window::q121_best_time_to_buy_and_sell_stock::Solution;

    #[test]
    fn test_max_profit() {
        assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
    }

    #[test]
    fn test_max_profit_no_profit() {
        assert_eq!(Solution::max_profit(vec![7, 6, 4, 3, 1]), 0);
    }

    #[test]
    fn test_max_profit_edge_case() {
        assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4, 0, 9]), 9);
    }
}
