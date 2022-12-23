use anchor_lang::prelude::*;

use crate::{constants::*, state::*};

#[derive(Accounts)]
pub struct Double<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init_if_needed,
        space = Counter::LEN,
        seeds = [COUNTER_TAG.as_ref()],
        bump,
        payer = signer
    )]
    pub counter: Account<'info, Counter>,

    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<Double>) -> Result<()> {
    let counter = &mut ctx.accounts.counter;

    counter.number *= 2;

    Ok(())
}
