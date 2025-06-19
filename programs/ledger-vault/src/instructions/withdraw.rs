use anchor_lang::prelude::*;
use anchor_spl::token::{
    Mint, 
    Token, 
    TokenAccount,
    Transfer,
    CloseAccount,
    close_account,
    transfer
};

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
    )]
    pub vault: Account<'info, TokenAccount>, 
    #[account(mut, close = user_ata)]   // HELP: why is it working without constraints ??
    pub vault_state: Account<'info, Vault>,
    pub token_program: Program<'info, Token>,
}

impl<'info> Withdraw<'info> {
    pub fn withdraw(&mut self) -> Result<()> {

        // Empty the vault back to the user (Cpi context needs signer seeds as it is being transferred out of a PDA)
        let cpi_program = self.token_program.to_account_info();
        
        let cpi_accounts = Transfer {
            from: self.vault.to_account_info(),
            to: self.user_ata.to_account_info(),
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

        let amount: u64 = self.vault.amount; // fixme
        transfer(cpi_context, amount)?;


        // Close the vault state (tip, seacrh for a close macro) => automatically closes the vault
        


        // Close the vault account
        let cpi_context_close = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            CloseAccount {
                account: self.vault.to_account_info(),
                destination: self.user.to_account_info(),
                authority: self.vault_state.to_account_info(),
            },
            signer_seeds,
        );
        close_account(cpi_context_close)?;





        Ok(())
    }

    pub fn close_vault(&mut self) -> Result<()> {


        Ok(())
    }
}