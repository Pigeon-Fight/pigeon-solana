use anchor_lang::prelude::*;
use anchor_spl::{metadata::{Metadata as SplMetadata, mpl_token_metadata::accounts::MasterEdition}, token::{Mint, Token, TokenAccount}};
use mpl_token_metadata::accounts::Metadata;

use crate::{
    account::{ItemClassInfo, NftAttributes, NftClassInfo, ADMIN_KEY}, CustomError, NewItemClassData, NewNftClassData
};

#[derive(Accounts)]
#[instruction(nft_class: u8, nft_data: NewNftClassData)]
pub struct SetNftData<'info> {
    #[account(
        mut,
        address = ADMIN_KEY @ CustomError::Unauthorized
    )]
    pub admin: Signer<'info>, // Only the admin can set the price
    #[account(
        init_if_needed,
        payer = admin,
        seeds = [b"nft_data", nft_class.to_le_bytes().as_ref()],
        bump,
        space = 16 // 8 (discriminator) + 8 bytes for price (u64)
    )]
    pub nft_class_info: Account<'info, NftClassInfo>, // PDA for the price of the class
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(item_class: u8, item_data: NewItemClassData)]
pub struct SetItemData<'info> {
    #[account(
        mut,
        address = ADMIN_KEY @ CustomError::Unauthorized
    )]
    pub admin: Signer<'info>, // Only the admin can set the price
    #[account(
        init_if_needed,
        payer = admin,
        seeds = [b"item_data", item_class.to_le_bytes().as_ref()],
        bump,
        space = 16 // 8 (discriminator) + 8 bytes for price (u64)
    )]
    pub item_class_info: Account<'info, ItemClassInfo>, // PDA for the price of the class
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(nft_class: u8)]
pub struct MintNft<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    /// CHECK: admin verify
    #[account(
        mut,
        address = ADMIN_KEY @ CustomError::Unauthorized
    )]
    pub admin: AccountInfo<'info>,
    #[account(
        init,
        payer = user,
        mint::decimals = 0,
        mint::authority = user.key(),
        mint::freeze_authority = user.key(),
    )]
    pub mint: Account<'info, Mint>,
    #[account(
        init,
        payer = user,
        token::mint = mint, 
        token::authority = user,
    )]
    pub token: Account<'info, TokenAccount>,
    /// CHECK: check metadata account
    #[account(
        init,
        payer = user,
        address = Metadata::find_pda(&mint.key()).0,
        space = 607 // https://developers.metaplex.com/token-metadata/guides/account-size-reduction
    )]
    pub metadata_account: AccountInfo<'info>,
    /// CHECK: check master edition account
    #[account(
        init,
        payer = user,
        address = MasterEdition::find_pda(&mint.key()).0,
        space = 20 // https://developers.metaplex.com/token-metadata/guides/account-size-reduction
    )]
    pub master_edition_account: AccountInfo<'info>,
    #[account(
        init,
        payer = user,
        seeds = [b"attributes", token.key().as_ref()],
        bump,
        space = 27 // u8 (1 byte) + u16 (2 bytes) * 9 => 8 (discriminator) + 19 (data)
    )]
    pub nft_attributes_account: Account<'info, NftAttributes>,

    #[account(
        seeds = [b"nft_data", nft_class.to_le_bytes().as_ref()],
        bump,
    )]
    pub nft_info_account: Account<'info, NftClassInfo>,
    
    pub token_metadata_program: Program<'info, SplMetadata>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct UpgradeNftAttributes<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        mut,
        seeds = [b"attributes", token.key().as_ref()],
        bump,
    )]
    pub nft_attribute_account: Account<'info, NftAttributes>,
    #[account(
        constraint = token.key() == owner.key() @ CustomError::NotOwner
    )]
    pub token: Account<'info, TokenAccount>
}

#[derive(Accounts)]
pub struct Battle<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        constraint = my_token.key() == owner.key() @ CustomError::NotOwner
    )]
    pub my_token: Account<'info, TokenAccount>,
    #[account(
        constraint = op_token.owner != my_token.key() @ CustomError::InvalidDefender
    )]
    pub op_token: Account<'info, TokenAccount>,
    #[account(
        mut,
        seeds = [b"attributes", my_token.key().as_ref()],
        bump,
    )]
    pub my_attributes: Account<'info, NftAttributes>,
    #[account(
        mut,
        seeds = [b"attributes", op_token.key().as_ref()],
        bump,
    )]
    pub op_attributes: Account<'info, NftAttributes>,
}


#[derive(Accounts)]
#[instruction(item_class: u8)]
pub struct PurchaseItem<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    /// CHECK: admin verify
    #[account(
        mut,
        address = ADMIN_KEY @ CustomError::Unauthorized
    )]
    pub admin: AccountInfo<'info>,
    #[account(
        constraint = token.key() == user.key() @ CustomError::NotOwner
    )]
    pub token: Account<'info, TokenAccount>,
    #[account(
        mut,
        seeds = [b"attributes", token.key().as_ref()],
        bump,
    )]
    pub nft_attributes_account: Account<'info, NftAttributes>,
    #[account(
        seeds = [b"item_data", item_class.to_le_bytes().as_ref()],
        bump,
    )]
    pub item_info: Account<'info, ItemClassInfo>
}