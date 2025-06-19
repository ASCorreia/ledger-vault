#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{Mint, Token, TokenAccount}};

use crate::state::Vault;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        init,
        payer = user,
        space = Vault::INIT_SPACE,
        seeds = [b"vault", user.key().as_ref(), mint.key().as_ref()],
        bump,
    )]
    pub vault_state: Account<'info, Vault>,
    pub mint: Account<'info, Mint>,
    #[account(
        init,
        payer = user,
        associated_token::mint = mint,
        associated_token::authority = vault_state,
    )]
    pub vault: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
}

impl<'info> Initialize<'info> {
    pub fn initialize(&mut self, bumps: InitializeBumps) -> Result<()> {

        self.vault_state.vault_bump = bumps.vault_state;

        Ok(())
    }
}