#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;

mod instructions;
mod state;

use instructions::*;

declare_id!("AV2VCiR1gr57BinWgxKFob915jbdwHkPkCVAjMci3Hsn");

#[program]
pub mod ledger_vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.initialize(ctx.bumps)
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        ctx.accounts.deposit(amount)
    }

    pub fn withdraw(ctx: Context<Withdraw>) -> Result<()> {
        ctx.accounts.withdraw()
    }

}

