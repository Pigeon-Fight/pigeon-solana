use crate::{account::NftAttributes, battle::internal_battle, error::CustomError};
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, TokenAccount};

#[derive(Accounts)]
pub struct Pvp<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account()]
    pub mint: Account<'info, Mint>,
    #[account(
        associated_token::mint = mint,
        associated_token::authority = owner
    )]
    pub token: Account<'info, TokenAccount>,
    #[account(
        constraint = mint.key() != op_mint.key() @ CustomError::InvalidDefender
    )]
    pub op_mint: Account<'info, Mint>,
    #[account(
        mut,
        seeds = [b"attributes", mint.key().as_ref()],
        bump,
    )]
    pub attributes: Account<'info, NftAttributes>,
    #[account(
        mut,
        seeds = [b"attributes", op_mint.key().as_ref()],
        bump,
    )]
    pub op_attributes: Account<'info, NftAttributes>,
}

impl<'info> Pvp<'info> {
    pub fn pvp(&mut self) -> Result<()> {
        let my_attribute_account = &mut self.attributes;
        let op_attribute_account = &mut self.op_attributes;

        if op_attribute_account.hp == 0 || my_attribute_account.hp == 0 {
            return Err(CustomError::InvalidDefender.into());
        }

        internal_battle(my_attribute_account, op_attribute_account);

        Ok(())
    }
}
