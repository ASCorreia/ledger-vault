#![allow(unexpected_cfgs)]
use crate::state::Vault;
use anchor_lang::prelude::*;
use anchor_spl::token::{
    close_account, transfer, CloseAccount, Mint, Token, TokenAccount, Transfer,
};

#[derive(Accounts)]
pub struct Withdraw<'info> {
    pub user: Signer<'info>,
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub user_ata: Account<'info, TokenAccount>,
    #[account(
        mut,
        close = user_ata,
        associated_token::mint = mint,
        associated_token::authority = vault_state,
    )]
    pub vault: Account<'info, TokenAccount>,
    #[account(
        mut,
        close = user,
    )]
    pub vault_state: Account<'info, Vault>,
    pub token_program: Program<'info, Token>,
}

impl<'info> Withdraw<'info> {
    pub fn withdraw(&mut self) -> Result<()> {
        let seeds = &[
            b"vault",
            self.user.to_account_info().key.as_ref(),
            self.mint.to_account_info().key.as_ref(),
            &[self.vault_state.vault_bump],
        ];

        let signer_seeds = &[&seeds[..]];

        transfer(
            CpiContext::new_with_signer(
                self.token_program.to_account_info(),
                Transfer {
                    from: self.vault.to_account_info(),
                    to: self.user_ata.to_account_info(),
                    authority: self.vault_state.to_account_info(),
                },
                signer_seeds,
            ),
            self.vault.amount,
        )?;

        close_account(CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            CloseAccount {
                account: self.vault.to_account_info(),
                destination: self.user_ata.to_account_info(),
                authority: self.vault_state.to_account_info(),
            },
            signer_seeds,
        ))?;

        Ok(())
    }

    pub fn close_vault(&mut self) -> Result<()> {
        Ok(())
    }
}
