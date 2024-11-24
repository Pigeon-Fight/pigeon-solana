use crate::{account::NftAttributes, error::CustomError};
use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;

use std::cmp;

fn attack(attacker: &mut NftAttributes, defender: &mut NftAttributes) {
    // energy run out => weak status
    if attacker.energy <= 0 {
        attacker.attack /= 2;
        attacker.defense /= 2;
    }

    // hp_lost min should be 1
    let hp_lost = if attacker.attack > defender.defense {
        attacker.attack - defender.defense
    } else {
        1
    };

    // take hp from defender
    if defender.hp > hp_lost {
        defender.hp -= hp_lost;
    } else {
        defender.hp = 0
    };

    // Every attack consume 1 energy
    if attacker.energy > 0 {
        attacker.energy -= 1;
    }
}

fn turn(
    action_threshold: u16,
    mut my_action_points: u16,
    mut op_action_points: u16,
    my: &mut NftAttributes,
    op: &mut NftAttributes,
) -> bool {
    my_action_points += my.speed;
    op_action_points += op.speed;

    if my_action_points >= action_threshold {
        if my.hp > 0 {
            attack(my, op);
            my_action_points -= action_threshold;
        }
    }

    if op_action_points >= action_threshold {
        if op.hp > 0 {
            attack(op, my);
            op_action_points -= action_threshold;
        }
    }

    if my.hp == 0 {
        // lose
        return false;
    }
    if op.hp == 0 {
        // win
        return true;
    }

    return turn(action_threshold, my_action_points, op_action_points, my, op);
}

fn internal_battle(my: &mut NftAttributes, op: &mut NftAttributes) -> bool {
    let action_threshold = cmp::max(my.speed, op.speed);

    // first turn
    let result = turn(action_threshold, 0, 0, my, op);

    // gain xp if win
    let gain_xp = (10 * (540 + op.exp.abs_diff(my.exp))) / 540;

    if result {
        my.exp += gain_xp;
    } else {
        op.exp += gain_xp;
    }

    return result;
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

impl<'info> Battle<'info> {
    pub fn battle(&mut self) -> Result<()> {
        let my_attribute_account = &mut self.my_attributes;
        let op_attribute_account = &mut self.op_attributes;

        let result = internal_battle(my_attribute_account, op_attribute_account);
        msg!("Battle: {}", result);

        Ok(())
    }
}
