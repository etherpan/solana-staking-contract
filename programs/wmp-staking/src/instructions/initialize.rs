use anchor_lang::{
    prelude::*, 
    solana_program::entrypoint::ProgramResult
};

use crate::state::*;

pub fn handler(ctx: Context<Initialize>) -> ProgramResult {
    let global_data = &mut ctx.accounts.global_data;

    global_data.bump = *ctx.bumps.get("global_data").unwrap();
    global_data.id = 0;

    Ok(())
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        init, 
        payer = admin,
        space = GLOBAL_DATA_SIZE,
        seeds=[GLOBAL_DATA_PREFIX.as_bytes()], 
        bump
    )]
    pub global_data: Box<Account<'info, GlobalData>>,

    pub system_program: Program<'info, System>
}