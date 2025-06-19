#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount, CloseAccount, close_account};

use crate::state::Vault;

#[derive(Accounts)]
pub struct Withdraw<'info> {
    pub user: Signer<'info>,
    pub mint: Account<'info, Mint>,
    #[account(
        mut
    )]
    pub user_ata: Account<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = vault_state,
    )]
    pub vault: Account<'info, TokenAccount>,
    #[account(
        mut,
        close = vault_state,
    )]
    pub vault_state: Account<'info, Vault>,
    pub token_program: Program<'info, Token>,
}

impl<'info> Withdraw<'info> {
    pub fn withdraw(&mut self) -> Result<()> {

        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = CloseAccount {
            account: self.vault.to_account_info(),
            destination: self.user_ata.to_account_info(),
            authority: self.vault_state.to_account_info(),
        };

        let seeds = &[
            b"vault",
            self.user.to_account_info().key.as_ref(),
            self.mint.to_account_info().key.as_ref(),
            &[self.vault_state.vault_bump],
        ];

        let signer_seeds = &[&seeds[..]];

        let cpi_context = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        close_account(cpi_context)?;

        Ok(())
    }

    pub fn close_vault(&mut self) -> Result<()> {


        Ok(())
    }
}