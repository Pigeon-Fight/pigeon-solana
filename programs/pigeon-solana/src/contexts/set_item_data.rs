use crate::{account::PigeonConfig, data::NewItemClassData, error::CustomError};
use anchor_lang::prelude::*;

#[account]
pub struct ItemClassInfo {
    pub price: u64,
    pub heal_hp: u16,
    pub heal_energy: u16,
    pub boost_attack: u16,
    pub boost_defense: u16,
    pub boost_speed: u16,
}

#[derive(Accounts)]
#[instruction(item_class: u8, item_data: NewItemClassData)]
pub struct SetItemData<'info> {
    #[account(
        mut,
        address = pigeon_config.admin_key @ CustomError::Unauthorized
    )]
    pub admin: Signer<'info>, // Only the admin can set the price
    #[account(
        init_if_needed,
        payer = admin,
        seeds = [b"item_data", item_class.to_le_bytes().as_ref()],
        bump,
        space = 26 // 8 (discriminator) + 8 bytes for price (u64) + 10 bytes attribute
    )]
    pub item_class_info: Account<'info, ItemClassInfo>, // PDA for the price of the class
    #[account(
        seeds = [b"pigeon_config"],
        bump
    )]
    pub pigeon_config: Account<'info, PigeonConfig>,
    pub system_program: Program<'info, System>,
}

impl<'info> SetItemData<'info> {
    pub fn set_item_data(&mut self, _item_class: u8, item_data: NewItemClassData) -> Result<()> {
        let item_class_info_account = &mut self.item_class_info;
        // Update Item Info
        item_class_info_account.price = item_data.price;
        item_class_info_account.heal_hp = item_data.heal_hp;
        item_class_info_account.heal_energy = item_data.heal_energy;
        item_class_info_account.boost_attack = item_data.boost_attack;
        item_class_info_account.boost_defense = item_data.boost_defense;
        item_class_info_account.boost_speed = item_data.boost_speed;
        Ok(())
    }
}
