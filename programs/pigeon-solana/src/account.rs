use anchor_lang::prelude::*;
pub const ADMIN_KEY: Pubkey = pubkey!("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v");

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

#[account]
pub struct NftClassInfo {
    pub price: u64,
    pub boost_attack: u16,
    pub boost_defense: u16,
    pub boost_speed: u16,
}

#[account]
pub struct ItemClassInfo {
    pub price: u64,
    pub health_hp: u16,
    pub health_energy: u16,
    pub boost_attack: u16,
    pub boost_defense: u16,
    pub boost_speed: u16,
}
