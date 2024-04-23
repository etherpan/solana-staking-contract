use anchor_lang::{prelude::*, solana_program::entrypoint::ProgramResult};
use anchor_spl::token::{Token, TokenAccount, Mint, Transfer, transfer};

use crate::{rewards::update_rewards, state::*};

pub fn handler(ctx: Context<ClaimRewards>) -> ProgramResult {
    let stake_pool_account_info = ctx.accounts.stake_pool.clone().to_account_info();

    let stake_pool = &mut ctx.accounts.stake_pool;
    let stake_entry = &mut ctx.accounts.stake_entry;

    update_rewards(stake_pool, stake_entry)?;
    
    let rewards_amount = stake_entry.rewards;

    stake_entry.rewards = 0;

    let accounts = Transfer {
        from: ctx.accounts.escrow_b.to_account_info(),
        to: ctx.accounts.staker_b.to_account_info(),
        authority: stake_pool_account_info
    };

    let seeds = &[
        STAKE_POOL_PREFIX.as_bytes(),
        &stake_pool.id.to_le_bytes(),
        &[stake_pool.bump]
    ];
    let signer_seeds = &[&seeds[..]];

    let transfer_context = CpiContext::new_with_signer(ctx.accounts.token_program.to_account_info(), accounts, signer_seeds);

    transfer(transfer_context, rewards_amount)?;
    
    Ok(())
}

#[derive(Accounts)]
pub struct ClaimRewards<'info> {
    #[account(mut)]
    pub staker: Signer<'info>,

    #[account(
        mut,
        seeds = [STAKE_POOL_PREFIX.as_bytes(), &stake_pool.id.to_le_bytes()],
        bump = stake_pool.bump
    )]
    pub stake_pool: Account<'info, StakePool>,

    #[account(
        mut,
        seeds = [STAKE_ENTRY_PREFIX.as_bytes(), &stake_pool.key().to_bytes(), &staker.key().to_bytes()],
        bump = stake_entry.bump
    )]
    pub stake_entry: Account<'info, StakeEntry>,

    #[account(
        mut,
        token::authority = staker,
        token::mint = mint_b
    )]
    pub staker_b: Account<'info, TokenAccount>,

    #[account(
        mut,
        token::authority = stake_pool,
        token::mint = mint_b
    )]
    pub escrow_b: Account<'info, TokenAccount>,

    #[account(
        constraint = mint_b.key() == stake_pool.mint_b.key()
    )]
    pub mint_b: Account<'info, Mint>,

    pub token_program: Program<'info, Token>
}