use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
mod dao_voting {
    use super::*;

    pub fn submit_proposal(ctx: Context<SubmitProposal>, title: String, description: String, options: Vec<String>) -> Result<()> {
        let proposal = &mut ctx.accounts.proposal;
        proposal.title = title;
        proposal.description = description;
        proposal.options = options;
        proposal.state = ProposalState::Open;
        proposal.votes = vec![];
        Ok(())
    }

    pub fn cast_vote(ctx: Context<CastVote>, option_index: usize) -> Result<()> {
        let proposal = &mut ctx.accounts.proposal;
        let vote = VoterVote {
            voter: *ctx.accounts.voter.key,
            option_index,
        };
        proposal.votes.push(vote);
        Ok(())
    }

    pub fn close_proposal(ctx: Context<CloseProposal>) -> Result<()> {
        let proposal = &mut ctx.accounts.proposal;
        proposal.state = ProposalState::Closed;
        Ok(())
    }
}

#[account]
pub struct Proposal {
    pub title: String,
    pub description: String,
    pub options: Vec<String>,
    pub state: ProposalState,
    pub votes: Vec<VoterVote>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct VoterVote {
    pub voter: Pubkey,
    pub option_index: usize,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum ProposalState {
    Open,
    Closed,
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
    pub voter: Signer<'info>,
}

#[derive(Accounts)]
pub struct CloseProposal<'info> {
    #[account(mut)]
    pub proposal: Account<'info, Proposal>,
    pub proposer: Signer<'info>,
}
