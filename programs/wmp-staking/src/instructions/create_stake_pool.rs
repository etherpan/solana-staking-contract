use anchor_lang::{
    prelude::*,
    solana_program::entrypoint::ProgramResult
};
use anchor_spl::{token::{Token, Mint, TokenAccount}, associated_token::AssociatedToken};
use crate::state::{STAKE_POOL_PREFIX, REWARD_ESCROW_A_PREFIX, REWARD_ESCROW_B_PREFIX, REWARD_ESCROW_FEE_PREFIX};
use crate::state::*;

pub fn handler(ctx: Context<CreateStakePool>, fee: u64) -> ProgramResult {
    let global_data = &mut ctx.accounts.global_data;
    let stake_pool = &mut ctx.accounts.stake_pool;

    msg!(
        "create_stake_pool handler"
    );

    stake_pool.bump = *ctx.bumps.get("stake_pool").unwrap();
    stake_pool.id = global_data.id;
    stake_pool.mint_a = ctx.accounts.mint_a.key();
    stake_pool.mint_b = ctx.accounts.mint_b.key();
    stake_pool.escrow_a = ctx.accounts.escrow_a.key();
    stake_pool.escrow_b = ctx.accounts.escrow_b.key();
    // stake_pool.escrow_fee = ctx.accounts.escrow_fee.key();
    stake_pool.creator = ctx.accounts.creator.key();
    stake_pool.balance = 0;
    stake_pool.last_update_timestamp = Clock::get().unwrap().unix_timestamp;
    stake_pool.rewards_per_token_stored = 0;
    stake_pool.fee = fee;
    global_data.id += 1;

    Ok(())
}

#[derive(Accounts)]
pub struct CreateStakePool<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,

    pub mint_a: Box<Account<'info, Mint>>,

    pub mint_b: Box<Account<'info, Mint>>,

    #[account(mut)]
    pub global_data: Box<Account<'info, GlobalData>>,

    #[account(
        init,
        payer = creator,
        token::mint = mint_a,
        token::authority = stake_pool,
        seeds = [REWARD_ESCROW_A_PREFIX.as_bytes(), &mint_a.key().to_bytes(), &stake_pool.key().to_bytes()],
        bump
    )]
    pub escrow_a: Box<Account<'info, TokenAccount>>,
    
    #[account(
        init,
        payer = creator,
        token::mint = mint_b,
        token::authority = stake_pool,
        seeds = [REWARD_ESCROW_B_PREFIX.as_bytes(), &mint_b.key().to_bytes(), &stake_pool.key().to_bytes()],
        bump
    )]
    pub escrow_b: Box<Account<'info, TokenAccount>>,

    // #[account(
    //     init,
    //     payer = creator,
    //     token::mint = mint_a,
    //     token::authority = creator,
    //     seeds = [REWARD_ESCROW_FEE_PREFIX.as_bytes(), &mint_a.key().to_bytes(), &creator.key().to_bytes()],
    //     bump
    // )]
    // pub escrow_fee: Box<Account<'info, TokenAccount>>, 

    #[account(
        init,
        payer = creator,
        space = STAKE_POOL_SIZE,
        seeds = [
            STAKE_POOL_PREFIX.as_bytes(),
            &global_data.id.to_le_bytes()
        ],
        bump
    )]
    pub stake_pool: Box<Account<'info, StakePool>>,

    pub system_program: Program<'info, System>,

    pub token_program: Program<'info, Token>,

    pub associated_token_program: Program<'info, AssociatedToken>,

    pub rent: Sysvar<'info, Rent>
}