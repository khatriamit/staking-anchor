use anchor_lang::prelude::*;
use anchor_spl::token::Token;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod staking {
    use anchor_spl::token::MintTo;

    use super::*;

    pub fn stake(ctx: Context<Stake>, amount: u64) -> Result<()> {
        let stakeAmount = beef_amount;


        let cpi_ctx = CpiContext::new_with_signer(ctx.accounts.token_program.to_account_info(), token::MintTo{
            mint:,
            authority:,
            to:,
        }, &signer,);
        token::mint_to(cpi_ctx, stakeAmount);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Stake<'info> {
    pub token_program:Program<'info, Token>,
}
