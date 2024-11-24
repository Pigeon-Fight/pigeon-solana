pub mod account;
pub mod data;
pub mod error;

use anchor_lang::prelude::*;

declare_id!("GtJQXa6CQ4qX4GuM92v7DbissAbVwrQJVcx5a4YESiCS");

pub mod contexts;
pub use contexts::*;
pub use data::*;

#[program]
pub mod pigeon_battle {

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.initialize(&ctx.bumps)
    }

    pub fn mint_nft(ctx: Context<MintNFT>, nft_class: u8) -> Result<()> {
        ctx.accounts.mint_nft(&ctx.bumps, nft_class)
    }

    pub fn battle(ctx: Context<Battle>) -> Result<()> {
        ctx.accounts.battle()
    }

    pub fn set_item_data(
        ctx: Context<SetItemData>,
        item_class: u8,
        item_data: NewItemClassData,
    ) -> Result<()> {
        ctx.accounts.set_item_data(item_class, item_data)
    }

    pub fn set_nft_data(
        ctx: Context<SetNftData>,
        nft_class: u8,
        nft_data: NewNftClassData,
    ) -> Result<()> {
        ctx.accounts.set_nft_data(nft_class, nft_data)
    }

    pub fn upgrade_nft(ctx: Context<UpgradeNft>, points: AllocatedAttribute) -> Result<()> {
        ctx.accounts.upgrade_nft(points)
    }

    pub fn purchase_item(ctx: Context<PurchaseItem>, item_class: u8) -> Result<()> {
        ctx.accounts.purchase_item(item_class)
    }
}
