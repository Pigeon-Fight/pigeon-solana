use crate::{account::NftAttributes, battle::internal_battle, error::CustomError};
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, TokenAccount};

fn point_to_exp(point: u16) -> u16 {
    return ((point - 46) * 50 / 3) + 100;
}

#[derive(Accounts)]
pub struct Pve<'info> {
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
        mut,
        seeds = [b"attributes", mint.key().as_ref()],
        bump,
    )]
    pub attributes: Account<'info, NftAttributes>,
}

impl<'info> Pve<'info> {
    pub fn pve(&mut self, enemy: NftAttributes) -> Result<()> {
        let my_attribute_account = &mut self.attributes;
        let op_attribute_account = &mut enemy.clone();

        // Recalculate exp based on enemy strength
        op_attribute_account.exp = point_to_exp(
            op_attribute_account.attack
                + op_attribute_account.defense
                + op_attribute_account.speed
                + op_attribute_account.max_energy
                + op_attribute_account.max_hp,
        );

        if op_attribute_account.hp == 0 || my_attribute_account.hp == 0 {
            return Err(CustomError::InvalidDefender.into());
        }

        internal_battle(my_attribute_account, op_attribute_account);

        Ok(())
    }
}
