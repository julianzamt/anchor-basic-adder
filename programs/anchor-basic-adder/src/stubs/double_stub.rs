use anchor_lang::prelude::*;

use crate::state::*;

pub fn ix_logic(counter: &mut Account<Counter>) -> Result<()> {
    // Place business logic here
    counter.number *= 2;

    Ok(())
}
