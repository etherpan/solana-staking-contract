use anchor_lang::{solana_program::entrypoint::ProgramResult, prelude::*};

use crate::state::StakePool;

pub fn handler(ctx: Context<SetStakePoolRewards>, rewards_per_second: u64) -> ProgramResult {
    let stake_pool = &mut ctx.accounts.stake_pool;
    
    stake_pool.rewards_per_second = rewards_per_second;

    Ok(())
}

#[derive(Accounts)]
pub struct SetStakePoolRewards<'info> {
    #[account(
        mut,
        constraint = admin.key() == stake_pool.creator.key()
    )]
    pub admin: Signer<'info>,

    #[account(mut)]
    pub stake_pool: Account<'info, StakePool>
}