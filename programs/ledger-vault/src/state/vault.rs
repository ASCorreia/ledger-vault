use std::mem;

use anchor_lang::prelude::*;

#[account]
pub struct Vault {
    pub vault_bump: u8,
    pub counter: u8,
    pub user_info: Vec<UserInfo>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy)]
pub struct UserInfo{
    pub timestamp: i64,
    pub amount: u64,
}

impl Space for Vault {
    const INIT_SPACE: usize = 8 + 1 + 1 + 4 + mem::size_of::<UserInfo>();
}