use anchor_lang::prelude::*;

declare_id!("7P8w59FYC99vUNERdCv4x5uoJ2ZRLPgMf4yAUSCbFyjX");

pub mod constants;

pub mod instructions;

pub mod state;

pub mod errors;

pub mod utils;

use crate::instructions::*;

#[program]
pub mod anchor_basic_adder {
    use super::*;

    pub fn add(ctx: Context<Add>, number: u32) -> Result<()> {
        add::handler(ctx, number)
    }

    pub fn double(ctx: Context<Double>) -> Result<()> {
        double::handler(ctx)
    }
}
