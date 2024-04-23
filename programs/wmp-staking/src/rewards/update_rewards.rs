use anchor_lang::{solana_program::entrypoint::ProgramResult, prelude::{Clock, SolanaSysvar}};
use crate::state::{StakePool, StakeEntry};
use super::{calculate_rewards_per_token, calculate_rewards};

pub fn update_rewards(stake_pool: &mut StakePool, stake_entry: &mut StakeEntry) -> ProgramResult {
    update_stake_pool(stake_pool)?;
    update_stake_entry(stake_pool, stake_entry)?;
    
    Ok(())
}

fn update_stake_pool(stake_pool: &mut StakePool) -> ProgramResult {
    let now = Clock::get().unwrap().unix_timestamp;
    stake_pool.rewards_per_token_stored = calculate_rewards_per_token(
        stake_pool.rewards_per_token_stored as u128, 
        stake_pool.rewards_per_second as u128, 
        now as u128, 
        stake_pool.last_update_timestamp as u128, 
        stake_pool.balance as u128)? as u64;

    stake_pool.last_update_timestamp = now;

    Ok(())
}

fn update_stake_entry(stake_pool: &StakePool, stake_entry: &mut StakeEntry) -> ProgramResult {
    stake_entry.rewards = calculate_rewards(
        stake_entry.rewards as u128, 
        stake_entry.balance as u128, 
        stake_pool.rewards_per_token_stored as u128, 
        stake_entry.rewards_per_token_paid as u128
    )? as u64;
    
    stake_entry.rewards_per_token_paid = stake_pool.rewards_per_token_stored;

    Ok(())
}