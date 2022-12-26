use anchor_lang::prelude::*;

use crate::state::*;

pub fn ix_logic(counter: &mut Account<Counter>, number: u32) -> Result<()> {
    // Place business logic here
    counter.number += number;

    Ok(())
}
