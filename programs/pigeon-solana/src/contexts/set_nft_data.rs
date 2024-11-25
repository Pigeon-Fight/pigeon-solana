use crate::{account::PigeonConfig, error::CustomError};
use anchor_lang::prelude::*;
use anchor_lang::{AnchorDeserialize, AnchorSerialize};

#[account]
pub struct NftClassInfo {
    pub price: u64,
    pub boost_attack: u16,
    pub boost_defense: u16,
    pub boost_speed: u16,
}

#[derive(Accounts)]
#[instruction(nft_class: u8, nft_data: NftClassInfo)]
pub struct SetNftData<'info> {
    #[account(
        mut,
        address = pigeon_config.admin_key @ CustomError::Unauthorized
    )]
    pub admin: Signer<'info>, // Only the admin can set the price
    #[account(
        init_if_needed,
        payer = admin,
        seeds = [b"nft_data", nft_class.to_le_bytes().as_ref()],
        bump,
        space = 22 // 8 (discriminator) + 8 bytes for price (u64) + 6 bytes attributes
    )]
    pub nft_class_info: Account<'info, NftClassInfo>, // PDA for the price of the class
    #[account(
        seeds = [b"pigeon_config"],
        bump
    )]
    pub pigeon_config: Account<'info, PigeonConfig>,
    pub system_program: Program<'info, System>,
}

impl<'info> SetNftData<'info> {
    pub fn set_nft_data(&mut self, _nft_class: u8, nft_data: NftClassInfo) -> Result<()> {
        let nft_class_info_account = &mut self.nft_class_info;
        // Update NFT info
        nft_class_info_account.price = nft_data.price;
        nft_class_info_account.boost_attack = nft_data.boost_attack;
        nft_class_info_account.boost_defense = nft_data.boost_defense;
        nft_class_info_account.boost_speed = nft_data.boost_speed;
        Ok(())
    }
}
