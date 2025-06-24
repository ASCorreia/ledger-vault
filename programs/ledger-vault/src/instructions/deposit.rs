use std::mem;

use anchor_lang::{prelude::*, system_program};
use anchor_spl::token::{
    transfer, 
    Mint, 
    Token, 
    TokenAccount, 
    Transfer
};

use crate::state::{
    UserInfo, 
    Vault
};

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    pub mint: Account<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = user,
    )]
    pub user_ata: Account<'info, TokenAccount>,
    #[account(
        mut,
        seeds = [b"vault", user.key().as_ref(), mint.key().as_ref()],
        bump = vault_state.vault_bump,
    )]
    pub vault_state: Account<'info, Vault>,
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = vault_state,
    )]
    pub vault: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

impl<'info> Deposit<'info> {
    pub fn deposit(&mut self, amount: u64) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = Transfer {
            from: self.user_ata.to_account_info(),
            to: self.vault.to_account_info(),
            authority: self.user.to_account_info(),
        };

        let cpi_context = CpiContext::new(cpi_program, cpi_accounts);

        transfer(cpi_context, amount)?;

        Ok(())
    }

    pub fn realloc(&self) -> Result<()> {
        let account_info = self.vault_state.to_account_info();
        let new_account_size = account_info.data_len() + std::mem::size_of::<UserInfo>();

        // Determine additional rent required
        let lamports_required = (Rent::get()?).minimum_balance(new_account_size);
        let additional_rent_to_fund = lamports_required - account_info.lamports();

        // Perform transfer of additional rent
        let cpi_program = self.system_program.to_account_info();
        let cpi_accounts = system_program::Transfer{
            from: self.user.to_account_info(), 
            to: account_info.clone(),
        };
        let cpi_context = CpiContext::new(cpi_program, cpi_accounts);
        system_program::transfer(cpi_context,additional_rent_to_fund)?;

        // Reallocate the account
        account_info.resize(new_account_size)?;
        msg!("Account Size Updated");

        Ok(())
    }

    pub fn update_state(&mut self, amount: u64) -> Result<()> {
        // Create a new UserInfo instance and add it to the vault state
        let user_info = UserInfo {
            timestamp: Clock::get()?.unix_timestamp,
            amount,
        };

        self.vault_state.user_info.push(user_info);

        // Increment the counter
        self.vault_state.counter += 1;

        Ok(())
    }
}