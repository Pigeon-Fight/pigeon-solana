use std::cmp;

use crate::account::NftAttributes;

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

pub fn internal_battle(my: &mut NftAttributes, op: &mut NftAttributes) -> bool {
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
