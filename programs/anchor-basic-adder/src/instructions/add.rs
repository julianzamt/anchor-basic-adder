use anchor_lang::prelude::*;

use crate::{constants::*, state::*, stubs::*};

#[derive(Accounts)]
pub struct Add<'info> {
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

pub fn handler(ctx: Context<Add>, number: u32) -> Result<()> {
    let counter = &mut ctx.accounts.counter;

    add_stub::ix_logic(counter, number)?;

    Ok(())
}
