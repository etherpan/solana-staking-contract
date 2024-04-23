use anchor_lang::{prelude::ProgramError};
use solana_safe_math::{SafeMath};

use crate::state::WMP_DECIMALS;

pub fn calculate_rewards(rewards: u128, stake_amount: u128, current_rewards_per_token: u128, prev_rewards_per_token: u128) -> Result<u128, ProgramError> {
    rewards.safe_add(
        current_rewards_per_token
            .safe_sub(prev_rewards_per_token)?
            .safe_mul(stake_amount)?
            .safe_div(10_u128.pow(WMP_DECIMALS))?
    )
}

pub fn calculate_rewards_per_token(prev_rewards_per_token: u128, rewards_per_sec: u128, now_timestamp: u128, prev_timestamp: u128, total_staked: u128) -> Result<u128, ProgramError> {
    if total_staked == 0 {
        return Ok(prev_rewards_per_token);
    }

    let interval = (now_timestamp).safe_sub(prev_timestamp)?;
    prev_rewards_per_token.safe_add(
        interval
            .safe_mul(rewards_per_sec)?
            .safe_mul(10_u128.pow(WMP_DECIMALS))?
            .safe_div(total_staked)?
    )
}

#[cfg(test)]
mod tests {
    use std::time::{SystemTime, UNIX_EPOCH};

    use super::*;

    impl PoolEntry {
        pub fn new(rewards_per_sec: u128) -> Self {
            PoolEntry { rewards_per_sec, balance: 0, timestamp: 0, rewards_per_token_stored: 0}
        }
    }

    pub struct PoolEntry {
        pub balance: u128,
        pub timestamp: i64,
        pub rewards_per_token_stored: u128,
        pub rewards_per_sec: u128
    }

    impl RewardsEntry {
        pub fn new() -> Self {
            RewardsEntry { balance: 0, rewards: 0, rewards_per_token_paid: 0 }
        }
    }

    pub struct RewardsEntry {
        pub balance: u128,
        pub rewards: u128,
        pub rewards_per_token_paid: u128
    }

    fn stake(pool: &mut PoolEntry, rewards_entry: &mut RewardsEntry, amount: u128, timestamp: i64) {
        pool.rewards_per_token_stored = calculate_rewards_per_token(
            pool.rewards_per_token_stored, 
            pool.rewards_per_sec, 
            timestamp as u128, 
            pool.timestamp as u128, 
            pool.balance).unwrap();

        pool.balance += amount;
        pool.timestamp = timestamp;

        rewards_entry.rewards = calculate_rewards(
            rewards_entry.rewards, 
            rewards_entry.balance, 
            pool.rewards_per_token_stored, 
            rewards_entry.rewards_per_token_paid).unwrap();
        
        rewards_entry.balance += amount;
        rewards_entry.rewards_per_token_paid = pool.rewards_per_token_stored;
    }

    fn unstake(pool: &mut PoolEntry, rewards_entry: &mut RewardsEntry, amount: u128, timestamp: i64) {
        pool.rewards_per_token_stored = calculate_rewards_per_token(
            pool.rewards_per_token_stored, 
            pool.rewards_per_sec, 
            timestamp as u128, 
            pool.timestamp as u128, 
            pool.balance).unwrap();

        pool.balance -= amount;
        pool.timestamp = timestamp;

        rewards_entry.rewards = calculate_rewards(
            rewards_entry.rewards, 
            rewards_entry.balance, 
            pool.rewards_per_token_stored, 
            rewards_entry.rewards_per_token_paid).unwrap();
        
        rewards_entry.balance -= amount;
        rewards_entry.rewards_per_token_paid = pool.rewards_per_token_stored;
    }

    fn get_rewards(pool: &mut PoolEntry, rewards_entry: &mut RewardsEntry, timestamp: i64) {
        pool.rewards_per_token_stored = calculate_rewards_per_token(
            pool.rewards_per_token_stored, 
            pool.rewards_per_sec, 
            timestamp as u128, 
            pool.timestamp as u128, 
            pool.balance).unwrap();

        pool.timestamp = timestamp;

        rewards_entry.rewards = calculate_rewards(
            rewards_entry.rewards, 
            rewards_entry.balance, 
            pool.rewards_per_token_stored, 
            rewards_entry.rewards_per_token_paid).unwrap();
        
        rewards_entry.rewards_per_token_paid = pool.rewards_per_token_stored;
    }


    #[test]
    fn test_1() {
        let rewards_per_token = 1.5_f64.token_amount();
        let rewards_per_sec = 1.0_f64.token_amount();
        let now_timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let prev_timestamp = now_timestamp - 3600;
        let total_staked = 100_u128.token_amount();
        let mut result = calculate_rewards_per_token(rewards_per_token, rewards_per_sec, now_timestamp as u128, prev_timestamp as u128, total_staked).unwrap();
        assert_eq!(37.5, result.human_amount());
    }

    #[test]
    fn test_2() {
        let prev_rewards = 100_u128.token_amount();
        let user_stake = 10_u128.token_amount();
        let rewards = calculate_rewards(prev_rewards, user_stake, 2_u128.token_amount(), 1_u128.token_amount())
            .unwrap()
            .human_amount();
        assert_eq!(110.0, rewards);
    }

    #[test]
    fn test_3() {
        let rewards_per_sec = 1.0_f64.token_amount();
        let pool = &mut PoolEntry::new(rewards_per_sec);

        let alice = &mut RewardsEntry::new();
        let tom = &mut RewardsEntry::new();
        let bob = &mut RewardsEntry::new();

        stake(pool, alice, 100.token_amount(), 10);
        stake(pool, bob, 200.token_amount(), 20);
        stake(pool, alice, 100.token_amount(), 40);
        stake(pool, tom, 500.token_amount(), 40);
        stake(pool, alice, 150.token_amount(), 80);
        unstake(pool, tom, 500.token_amount(), 90);

        get_rewards(pool, alice, 120);
        get_rewards(pool, bob, 120);

        assert_eq!(pool.rewards_per_token_stored.human_amount(), 0.275180373);

        assert_eq!(alice.rewards.human_amount(), 47.97979745);
        assert_eq!(bob.rewards.human_amount(), 35.0360746);
        assert_eq!(tom.rewards.human_amount(), 26.9841265);
    }
}

pub trait TokenAmount  {
    fn token_amount(&mut self) -> u128;
    fn human_amount(&mut self) -> f64;
}

impl TokenAmount for f64 {
    fn token_amount(&mut self) -> u128 {
        const DECIMALS: u32 = 9;
        let f64_res = *self * 10_u64.pow(DECIMALS) as f64;
        f64_res as u128
    }

    fn human_amount(&mut self) -> f64 {
        (*self as u128).human_amount()
    }
}

impl TokenAmount for u128 {
    fn token_amount(&mut self) -> Self {
        const DECIMALS: u32 = 9;
        *self * 10_u128.pow(DECIMALS)
    }

    fn human_amount(&mut self) -> f64 {
        const DECIMALS: u32 = 9;

        let u64_result = *self as f64 / 10_u128.pow(DECIMALS) as f64;
        u64_result
    }
}