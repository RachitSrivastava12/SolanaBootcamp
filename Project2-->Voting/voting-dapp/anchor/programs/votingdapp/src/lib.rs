// #![allow(clippy::result_large_err)]

// use anchor_lang::prelude::*;

// declare_id!("AsjZ3kWAUSQRNt2pZVeJkywhZ6gpLpHZmJjduPmKZDZZ");

// #[program]
// pub mod voting {
//     use super::*;

//     pub fn initialize_poll(
//         _ctx: Context<InitializePoll>, 
//         _poll_id: u64
//     ) -> Result<()> {
//         Ok(())
//     } 
// }

// #[account]
// #[derive(INIT_SPACE)] 
// pub struct Poll{
//     pub poll_id: u64,
//     #[max_len(320)]
//     pub description: String,
//     pub poll_start: u64,
//     pub poll_end: u64,
//     pub candidate_amount: u64, 
// }


// #[derive(Accounts)]
// #[instruction(poll_id: u64)]
// pub struct InitializePoll<'info> {
//     #[account(mut)]
//     pub signer: Signer<'info>,
//     #[account(
//         init,
//         payer =  signer,
//         space = 8 + Poll::INIT_SPACE,
//         seeds = [poll_id.to_le_bytes().as_ref()],
//         bump,
//     )]
//     pub poll:Account<'info, Poll>,

//     pub system_program: Program<'info, System>,

// }

#![allow(clippy::result_large_err)]
use anchor_lang::prelude::*;

declare_id!("AsjZ3kWAUSQRNt2pZVeJkywhZ6gpLpHZmJjduPmKZDZZ");

#[program]
pub mod votingdapp {
    use super::*;
    pub fn initialize_poll(
        ctx: Context<InitializePoll>,
        poll_id: u64,
        description: String,
        poll_start: u64,
        poll_end: u64,
       

    ) -> Result<()> {
  let poll=  &mut ctx.accounts.poll;
  poll.poll_id = poll_id;
  poll.poll_start = poll_start;
  poll.poll_end = poll_end;
  poll.candidate_amount = 0;
  poll.description = description;

        Ok(())
    }
}

// Changed from #[Account] to #[account]
#[account]

pub struct Poll {
    pub poll_id: u64,
    // Removed max_len attribute, it's handled in space parameter below
    pub description: String,
    pub poll_start: u64,
    pub poll_end: u64,
    pub candidate_amount: u64,
}

#[derive(Accounts)]
#[instruction(poll_id: u64)]
pub struct InitializePoll<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init,
        payer = signer,  // Changed from payer to signer
        space = 8 + 8 + 4 + 320 + 8 + 8 + 8,  // Added explicit space
        seeds = [poll_id.to_le_bytes().as_ref()],
        bump,
    )]
    pub poll: Account<'info, Poll>,
    
    pub system_program: Program<'info, System>,
}