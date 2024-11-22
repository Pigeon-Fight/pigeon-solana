use anchor_lang::prelude::*;
use anchor_lang::{AnchorDeserialize, AnchorSerialize};

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct AllocatedAttribute {
    pub attack: u16,
    pub defense: u16,
    pub speed: u16,
    pub max_hp: u16,
    pub max_energy: u16,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct NewNftClassData {
    pub price: u64,
    pub boost_attack: u16,
    pub boost_defense: u16,
    pub boost_speed: u16,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct NewItemClassData {
    pub price: u64,
    pub heal_hp: u16,
    pub heal_energy: u16,
    pub boost_attack: u16,
    pub boost_defense: u16,
    pub boost_speed: u16,
}
