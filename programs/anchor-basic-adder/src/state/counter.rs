use anchor_lang::prelude::*;

use crate::constants::*;

#[account]
#[derive(Default)]
pub struct Counter {
    pub number: u32,
}

impl Counter {
    pub const LEN: usize = DISCRIMINATOR_LENGTH + 4;
}
