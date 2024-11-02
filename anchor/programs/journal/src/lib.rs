#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;

declare_id!("AsjZ3kWAUSQRNt2pZVeJkywhZ6gpLpHZmJjduPmKZDZZ");

// all the instructions are going to live under program
#[program]
pub mod journal {
    use super::*;

    pub fn create_entry(ctx: Context<CreateEntry>, title: String, message: String) -> Result<()> {
        let journal_entry = &mut ctx.accounts.journal_entry;
        journal_entry.owner = ctx.accounts.owner.key();
        journal_entry.title = title;
        journal_entry.message = message;
        // journal_entry.entry_id = ctx.accounts.journal_entry.to_account_info().key().to_bytes()[0];
        Ok(())
    }

    // want to be able to update the entry after it's been initialised
    pub fn update_entry(ctx: Context<UpdateEntry>, title: String, message: String) -> Result<()> {
        let journal_entry = &mut ctx.accounts.journal_entry;
        journal_entry.title = title;
        journal_entry.message = message;
        Ok(())
    }
 
}

// State is a data structure where your what all your program information saved
// data is stored on accounts that live on the blockchain. We use an "account" macro
#[account]
#[derive(InitSpace)]  
pub struct JournalEntryState {
   pub owner: Pubkey,
   #[max_len(20)]
   pub title: String,
   #[max_len(200)]
   pub message: String,
   pub entry_id: u64,
}

#[derive(Accounts)] 
#[instruction(title: String)]
pub struct CreateEntry<'info> {
    #[account(
      init, 
      seeds = [title.as_bytes(), owner.key().as_ref()],
      bump,
      payer = owner, // who's going to pay for the account creation
      space = 8 + JournalEntryState::INIT_SPACE,
      )]
   
    pub journal_entry: Account<'info, JournalEntryState>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)] 
#[instruction(title: String)]
pub struct UpdateEntry<'info> {
    #[account(
      init, 
      seeds = [title.as_bytes(), owner.key().as_ref()],
      bump,
      payer = owner, // who's going to pay for the account creation
      space = 8 + JournalEntryState::INIT_SPACE,
      )]
   
    pub journal_entry: Account<'info, JournalEntryState>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}
