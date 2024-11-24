use anchor_lang::prelude::*;

#[account]
pub struct PigeonConfig {
    pub admin_key: Pubkey,
}

#[account]
pub struct NftAttributes {
    pub class: u8,
    pub hp: u16,
    pub energy: u16,
    pub exp: u16,
    pub allocated_point: u16,
    pub attack: u16,
    pub defense: u16,
    pub speed: u16,
    pub max_hp: u16,
    pub max_energy: u16,
}
