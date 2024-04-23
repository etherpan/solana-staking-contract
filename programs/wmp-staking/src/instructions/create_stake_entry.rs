use anchor_lang::{
    prelude::*, 
    solana_program::entrypoint::ProgramResult, Accounts
};

use crate::state::{GlobalData, StakePool, StakeEntry, stake_entry::*};

pub fn handler(ctx: Context<CreateStakeEntry>) -> ProgramResult {
    let stake_entry = &mut ctx.accounts.stake_entry;

    stake_entry.bump = *ctx.bumps.get("stake_entry").unwrap();
    stake_entry.pool = ctx.accounts.stake_pool.key();
    stake_entry.balance = 0;
    stake_entry.rewards = 0;
    stake_entry.rewards_per_token_paid = 0;

    Ok(())
}

#[derive(Accounts)]
pub struct CreateStakeEntry<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    pub global_data: Account<'info, GlobalData>,

    pub stake_pool: Account<'info, StakePool>,

    #[account(
        init,
        payer = user,
        space = STAKE_ENTRY_SIZE,
        seeds = [
            STAKE_ENTRY_PREFIX.as_bytes(),
            &stake_pool.key().to_bytes(),
            &user.key().to_bytes()
        ],
        bump
    )]
    pub stake_entry: Account<'info, StakeEntry>,

    pub system_program: Program<'info, System>
}