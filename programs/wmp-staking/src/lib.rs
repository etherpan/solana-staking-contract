pub mod instructions;
pub mod state;
pub mod error;
pub mod rewards;

use anchor_lang::prelude::*;
use instructions::*;
use anchor_lang::solana_program::entrypoint::ProgramResult;

declare_id!("zCqMotQEKDpxkbv96kFS4p9XcCN19T48vtumdFe1GFd");

#[program]
pub mod wmp_staking {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        instructions::initialize::handler(ctx)
    }

    pub fn set_stake_pool_rewards(ctx: Context<SetStakePoolRewards>, rewards_per_second: u64) -> ProgramResult {
        instructions::set_stake_pool_rewards::handler(ctx, rewards_per_second)
    }

    pub fn create_stake_pool(ctx: Context<CreateStakePool>, fee: u64) -> ProgramResult {
        instructions::create_stake_pool::handler(ctx, fee)
    }

    pub fn create_stake_entry(ctx: Context<CreateStakeEntry>) -> ProgramResult {
        instructions::create_stake_entry::handler(ctx)
    }

    pub fn stake(ctx: Context<Stake>, amount: u64) -> ProgramResult {
        instructions::stake::handler(ctx, amount)
    }

    pub fn unstake(ctx: Context<Unstake>, amount: u64) -> ProgramResult {
        instructions::unstake::handler(ctx, amount)
    }

    pub fn claim_rewards(ctx: Context<ClaimRewards>) -> ProgramResult {
        instructions::claim_rewards::handler(ctx)
    }
}
