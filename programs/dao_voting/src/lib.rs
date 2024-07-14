use anchor_lang::prelude::*;

declare_id!("DrJcZH3mctarr6KXHyuEPcQWPSLWvzRtf86GRDTdAmiR");

#[program]
mod dao_voting {
    use super::*;

    pub fn initialize_voter(ctx: Context<InitializeVoter>) -> Result<()> {
        let voter = &mut ctx.accounts.voter;
        voter.reward_points = 0;
        Ok(())
    }

    pub fn submit_proposal(
        ctx: Context<SubmitProposal>,
        title: String,
        description: String,
        options: Vec<String>,
    ) -> Result<()> {
        let proposal = &mut ctx.accounts.proposal;
        proposal.title = title;
        proposal.description = description;
        proposal.options = options;
        proposal.state = ProposalState::Open;
        proposal.votes = vec![];
        Ok(())
    }

    pub fn cast_vote(ctx: Context<CastVote>, option_index: u8) -> Result<()> {
        let proposal = &mut ctx.accounts.proposal;
        let voter = &mut ctx.accounts.voter;

        let vote = ProposalVote {
            voter: *voter.to_account_info().key,
            option_index,
        };
        proposal.votes.push(vote);

        // Reward the voter with points
        voter.reward_points += 1;

        Ok(())
    }

    pub fn close_proposal(ctx: Context<CloseProposal>) -> Result<()> {
        let proposal = &mut ctx.accounts.proposal;
        proposal.state = ProposalState::Closed;
        Ok(())
    }

    pub fn get_proposal_results(ctx: Context<GetProposalResults>) -> Result<Vec<u64>> {
        let proposal = &ctx.accounts.proposal;

        // Ensure the proposal is closed before displaying results
        require!(
            proposal.state == ProposalState::Closed,
            ProposalError::ProposalStillOpen
        );

        let mut results = vec![0; proposal.options.len()];

        for vote in &proposal.votes {
            if (vote.option_index as usize) < results.len() {
                results[vote.option_index as usize] += 1;
            }
        }

        Ok(results)
    }
}

#[account]
pub struct Proposal {
    pub title: String,
    pub description: String,
    pub options: Vec<String>,
    pub state: ProposalState,
    pub votes: Vec<ProposalVote>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct ProposalVote {
    pub voter: Pubkey,
    pub option_index: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum ProposalState {
    Open,
    Closed,
}

#[account]
pub struct Voter {
    pub reward_points: u64,
}

#[derive(Accounts)]
pub struct InitializeVoter<'info> {
    #[account(init, payer = user, space = 8 + 8)]
    pub voter: Account<'info, Voter>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SubmitProposal<'info> {
    #[account(init, payer = proposer, space = 8 + 1024)]
    pub proposal: Account<'info, Proposal>,
    #[account(mut)]
    pub proposer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CastVote<'info> {
    #[account(mut)]
    pub proposal: Account<'info, Proposal>,
    #[account(mut)]
    pub voter: Account<'info, Voter>,
}

#[derive(Accounts)]
pub struct CloseProposal<'info> {
    #[account(mut)]
    pub proposal: Account<'info, Proposal>,
    pub proposer: Signer<'info>,
}

#[derive(Accounts)]
pub struct GetProposalResults<'info> {
    pub proposal: Account<'info, Proposal>,
}

#[error_code]
pub enum ProposalError {
    #[msg("Cannot get results for an open proposal")]
    ProposalStillOpen,
}
