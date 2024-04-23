use std::mem::size_of;
use anchor_lang::prelude::*;

pub const WMP_DECIMALS: u32 = 9;

pub const GLOBAL_DATA_SIZE: usize = size_of::<GlobalData>() + 8;
pub const GLOBAL_DATA_PREFIX: &str = "global-data";

#[account]
pub struct GlobalData {
    pub bump: u8,

    // TODO rename to count or next_id 
    pub id: u16
}