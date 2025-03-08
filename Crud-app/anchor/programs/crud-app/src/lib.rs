#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;

declare_id!("coUnmi3oBUtwtd9fjeAvSsJssXh5A5xyPbhpewyzRVF");

#[program]
pub mod Crudapp {
    use super::*;

    pub fn create_journal_entry(
      ctx: Context<CreateEntry>, 
      title: String, 
      message: String) -> ProgramResult {
        let journal_entry = &mut ctx.accounts.journal_entry;
        journal_entry.owner = *ctx.accounts.owner.key;
        journal_entry.title = title;
        journal_entry.message = message;
        Ok(())
    }
    
    pub fn update_journal_entry(
      ctx: Context<UpdateEntry>, 
      title: String, 
      message: String) -> ProgramResult {
        let journal_entry = &mut ctx.accounts.journal_entry;
        journal_entry.title = title;
        journal_entry.message = message;
        Ok(())
    }

    pub fn delete_journal_entry(
      ctx: Context<DeleteEntry>
      _title: String) -> ProgramResult {
        ctx.accounts.journal_entry.owner = Pubkey::default();
        ctx.accounts.journal_entry.title = "".to_string();
        ctx.accounts.journal_entry.message = "".to_string();
        Ok(())
    }
    
}

#[derive(Accounts)]
pub struct CreateEntry<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account( 
       init_if_needed,
       payer = owner,
       space = 8 + JournalEntryState::INIT_SPACE,
       seeds = [title.as_bytes(), &owner.key().as_ref()],
       bump 
      )]
    pub journal_entry: Account<'info, JournalEntryState>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(title: String)]
pub struct UpdateEntry<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
       mut,
       seeds = [title.as_bytes(), &owner.key().as_ref()],
       bump,
       realloc = 8 + JournalEntryState::INIT_SPACE
       realloc::payer = owner;
       realloc::zero = true;
      )]
    pub journal_entry: Account<'info, JournalEntryState>,

    pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
#[instruction(title: String)]
pub struct DeleteEntry<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
       mut,
       seeds = [title.as_bytes(), &owner.key().as_ref()],
       close = owner,
      )]
    pub journal_entry: Account<'info, JournalEntryState>,

    pub system_program: Program<'info, System>,
}
#[account]
#[derive(InitSpace)]
pub struct JournalEntryState {
    pub owner: Pubkey,
    #[max_len = 32]
    pub title: String,
    #[max_len = 100]
    pub message: String,
}