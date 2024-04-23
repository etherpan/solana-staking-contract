use std::mem::size_of;

use anchor_lang::{prelude::*};

pub const STAKE_ENTRY_SIZE: usize = size_of::<StakeEntry>() + 8;
pub const STAKE_ENTRY_PREFIX: &str = "stake-entry";

#[account]
pub struct StakeEntry {
    pub bump: u8,
    pub pool: Pubkey,
    pub balance: u64,
    pub rewards: u64,
    pub rewards_per_token_paid: u64
}