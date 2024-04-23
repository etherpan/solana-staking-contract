use anchor_lang::{prelude::*, solana_program::entrypoint::ProgramResult};
use anchor_spl::token::{Token, TokenAccount, Mint, Transfer, transfer};

use crate::rewards::update_rewards;
use crate::state::{StakePool, StakeEntry, STAKE_ENTRY_PREFIX, STAKE_POOL_PREFIX};
use crate::error::ErrorCode;

pub fn handler(ctx: Context<Stake>, amount: u64) -> ProgramResult {
    let stake_pool = &mut ctx.accounts.stake_pool;
    let stake_entry = &mut ctx.accounts.stake_entry;

    update_rewards(stake_pool, stake_entry)?;
    
    let fee_amount = amount * stake_pool.fee / 100;
    stake_pool.balance += amount - fee_amount;
    stake_entry.balance += amount - fee_amount;

    let accounts = Transfer {
        from: ctx.accounts.staker_token_a.to_account_info(),
        to: ctx.accounts.escrow_a.to_account_info(),
        authority: ctx.accounts.staker.to_account_info()
    };

    let transfer_context = CpiContext::new(ctx.accounts.token_program.to_account_info(), accounts);

    transfer(transfer_context, amount - fee_amount)?;


    let accounts = Transfer {
        from: ctx.accounts.staker_token_a.to_account_info(),
        to: ctx.accounts.escrow_fee.to_account_info(),
        authority: ctx.accounts.staker.to_account_info()
    };

    let transfer_context = CpiContext::new(ctx.accounts.token_program.to_account_info(), accounts);

    transfer(transfer_context, fee_amount)?;

    Ok(())
}

#[derive(Accounts)]
pub struct Stake<'info> {
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
        constraint = 
            staker_token_a.mint == stake_pool.mint_a &&
            staker_token_a.mint == mint_a.key() @ ErrorCode::TokenAMintMismatch,
        constraint = 
            staker_token_a.owner == staker.key() @ ErrorCode::OwnerMismatch
    )]
    pub staker_token_a: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = 
            escrow_a.mint == stake_pool.mint_a &&
            escrow_a.mint == mint_a.key() @ ErrorCode::TokenAMintMismatch,
        constraint = 
            escrow_a.owner == stake_pool.key() @ ErrorCode::OwnerMismatch
    )]
    pub escrow_a: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = 
            escrow_fee.mint == stake_pool.mint_a &&
            escrow_fee.mint == mint_a.key() @ ErrorCode::TokenAMintMismatch,
        constraint = 
            escrow_fee.owner == stake_pool.creator.key() @ ErrorCode::OwnerMismatch
    )]
    pub escrow_fee: Account<'info, TokenAccount>,

    pub mint_a: Account<'info, Mint>,

    pub token_program: Program<'info, Token>
}