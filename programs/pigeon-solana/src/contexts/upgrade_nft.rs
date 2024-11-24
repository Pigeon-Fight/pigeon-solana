use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;

use crate::{account::NftAttributes, data::AllocatedAttribute, error::CustomError};

const BASE_EXP: u16 = 10;
fn exp_to_point(exp: u16) -> u16 {
    return 3 * (exp - BASE_EXP) / 50;
}

#[derive(Accounts)]
pub struct UpgradeNft<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        mut,
        seeds = [b"attributes", token.key().as_ref()],
        bump,
    )]
    pub nft_attribute_account: Account<'info, NftAttributes>,
    #[account(
        constraint = token.owner.key() == owner.key() @ CustomError::NotOwner
    )]
    pub token: Account<'info, TokenAccount>,
}

impl<'info> UpgradeNft<'info> {
    pub fn upgrade_nft(&mut self, points: AllocatedAttribute) -> Result<()> {
        let nft_attribute_account = &mut self.nft_attribute_account;

        let total_point = exp_to_point(nft_attribute_account.exp);
        let remain_point = total_point - nft_attribute_account.allocated_point;
        let allocate_point =
            points.attack + points.defense + points.speed + points.max_hp + points.max_energy;
        if remain_point < allocate_point {
            return Err(CustomError::InvalidPoints.into());
        }

        // Update the attributes
        nft_attribute_account.attack += points.attack;
        nft_attribute_account.defense += points.defense;
        nft_attribute_account.speed += points.speed;
        nft_attribute_account.max_hp += points.max_hp;
        nft_attribute_account.max_energy += points.max_energy;

        // Evolute
        if nft_attribute_account.class <= 36 {
            // second evolution
            if nft_attribute_account.class <= 18 {
                if nft_attribute_account.allocated_point >= 310 {
                    nft_attribute_account.class *= 2;
                }
            }
            // first evolution
            if nft_attribute_account.class <= 10 {
                if nft_attribute_account.allocated_point >= 160 {
                    nft_attribute_account.class *= 2;
                }
            }
        }

        Ok(())
    }
}
