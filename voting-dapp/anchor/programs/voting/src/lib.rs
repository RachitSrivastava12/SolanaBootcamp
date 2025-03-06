use anchor_lang::prelude::*;
declare_id!("96EH7YWE6M6jusxeVnGdRsQGJMYaZZsYZuxfjj6tmqs9");

#[program]
pub mod voting {
    use super::*;
    
    pub fn initialize_poll(
        ctx: Context<InitializePoll>,
        poll_id: u64,
        description: String,
        poll_start: i64,
        poll_end: i64,
    ) -> Result<()> {
        let poll = &mut ctx.accounts.poll;
        poll.poll_id = poll_id;
        poll.poll_start = poll_start;
        poll.poll_end = poll_end;
        poll.description = description;
        poll.candidate_amount = 0;
        Ok(())
    }
    
    pub fn add_candidate(
        ctx: Context<AddCandidate>,
        candidate_name: String,
        poll_id: u64,
    ) -> Result<()> {
        let candidate = &mut ctx.accounts.candidate;
        candidate.candidate_name = candidate_name;
        candidate.candidate_votes = 0;
        
        // Increment the candidate amount in the poll
        let poll = &mut ctx.accounts.poll;
        poll.candidate_amount += 1;
        
        Ok(())
    }

    pub fn vote(ctx: Context<Vote>, candidate_name: String, poll_id: u64) -> Result<()> {
        let candidate = &mut ctx.accounts.candidate;
        candidate.candidate_votes += 1;
        Ok(())
    }
}
initialize_poll: u64;
instruction: Context<>
#[derive(Accounts)]
#[instruction(candidate_name: String, poll_id: u64)]
pub struct Vote<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        mut,
        seeds = [poll_id.to_le_bytes().as_ref()],
        bump,
    )]
    pub poll: Account<'info, Poll>,
    #[account(
        mut,
        seeds = [poll_id.to_le_bytes().as_ref(), candidate_name.as_bytes()],
        bump,
    )]
    pub candidate: Account<'info, Candidate>,
}

#[derive(Accounts)]
#[instruction(poll_id: u64)]
pub struct InitializePoll<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        init,
        payer = signer,
        space = 8 + Poll::INIT_SPACE,
        seeds = [poll_id.to_le_bytes().as_ref()],
        bump,
    )]
    pub poll: Account<'info, Poll>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(candidate_name: String, poll_id: u64)]
pub struct AddCandidate<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        mut,
        seeds = [poll_id.to_le_bytes().as_ref()],
        bump,
    )]
    pub poll: Account<'info, Poll>,
    #[account(
        init,
        payer = signer,
        space = 8 + Candidate::INIT_SPACE,
        seeds = [poll_id.to_le_bytes().as_ref(), candidate_name.as_bytes()],
        bump,
    )]
    pub candidate: Account<'info, Candidate>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct Poll {
    pub poll_id: u64,
    #[max_len(280)]
    pub description: String,
    pub poll_start: i64,
    pub poll_end: i64,
    pub candidate_amount: u64,
}

#[account]
#[derive(InitSpace)]
pub struct Candidate {
    #[max_len(32)]
    pub candidate_name: String,
    pub candidate_votes: u64,
}