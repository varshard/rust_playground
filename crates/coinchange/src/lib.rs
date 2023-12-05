use std::cmp::min;

pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
    let amount = amount as usize;
    let mut min_coin = vec![i32::MAX; 1 + amount];
    min_coin[0] = 0;

    for i in 1..=amount {
        for c in coins.iter() {
            let c = *c as usize;
            match i.checked_sub(c) {
                None => {}
                Some(left) => {
                    if min_coin[left] != i32::MAX {
                        min_coin[i] = min(min_coin[i], min_coin[left] + 1)
                    }
                }
            }
        }
    }

    if min_coin[amount] == i32::MAX {
        return -1;
    }

    return min_coin[amount];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(coin_change(vec![13, 8, 2], 15), 2);
        // assert_eq!(coin_change(vec![5, 10], 20), 2);
        // assert_eq!(coin_change(vec![5, 10], 3), -1);
        // assert_eq!(coin_change(vec![5, 10], 10), 1);
        // assert_eq!(coin_change(vec![5,8, 13], 100), 8);
    }
}
