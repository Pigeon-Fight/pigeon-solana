use anchor_lang::prelude::*;

declare_id!("GtJQXa6CQ4qX4GuM92v7DbissAbVwrQJVcx5a4YESiCS");

#[program]
pub mod pigeon_solana {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
