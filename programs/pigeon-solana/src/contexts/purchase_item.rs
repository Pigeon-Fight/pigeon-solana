use crate::{
    account::{NftAttributes, PigeonConfig},
    error::CustomError,
};
use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;

use super::ItemClassInfo;

#[derive(Accounts)]
#[instruction(item_class: u8)]
pub struct PurchaseItem<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    /// CHECK: admin verify
    #[account(
        mut,
        address = pigeon_config.admin_key @ CustomError::Unauthorized
    )]
    pub admin: AccountInfo<'info>,
    #[account(
        constraint = token.owner.key() == user.key() @ CustomError::NotOwner
    )]
    pub token: Account<'info, TokenAccount>,
    #[account(
        mut,
        seeds = [b"attributes", token.key().as_ref()],
        bump,
    )]
    pub nft_attributes_account: Account<'info, NftAttributes>,
    #[account(
        seeds = [b"pigeon_config"],
        bump,
    )]
    pub pigeon_config: Account<'info, PigeonConfig>,
    #[account(
        seeds = [b"item_data", item_class.to_le_bytes().as_ref()],
        bump,
    )]
    pub item_info: Account<'info, ItemClassInfo>,
}

impl<'info> PurchaseItem<'info> {
    pub fn purchase_item(&mut self, _item_class: u8) -> Result<()> {
        let item_info = &self.item_info;

        let user_account = &self.user;
        let sol_price = item_info.price; // Get the price from the PDA
        let user_balance = **user_account.try_borrow_lamports()?; // Check user's balance

        // Ensure the user has enough SOL
        if user_balance < sol_price {
            return Err(CustomError::InsufficientFunds.into());
        }

        let nft_attributes_account = &mut self.nft_attributes_account;

        nft_attributes_account.hp += nft_attributes_account.max_hp * item_info.heal_hp / 100;
        nft_attributes_account.energy +=
            nft_attributes_account.max_energy * item_info.heal_energy / 100;

        nft_attributes_account.attack += item_info.boost_attack;
        nft_attributes_account.defense += item_info.boost_defense;
        nft_attributes_account.speed += item_info.boost_speed;

        Ok(())
    }
}
