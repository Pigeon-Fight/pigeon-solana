pub mod account;
pub mod custom_accounts;
pub mod data;
pub mod error;
pub mod logic;

use custom_accounts::*;
use data::*;
use error::*;
use logic::*;

use anchor_lang::prelude::*;
use anchor_lang::solana_program::entrypoint::ProgramResult;
use anchor_lang::solana_program::program::invoke;
use anchor_lang::solana_program::system_instruction;
use anchor_spl::metadata::mpl_token_metadata::types::DataV2;
use anchor_spl::{
    metadata::{
        create_master_edition_v3, create_metadata_accounts_v3, CreateMasterEditionV3,
        CreateMetadataAccountsV3,
    },
    token,
};

declare_id!("GtJQXa6CQ4qX4GuM92v7DbissAbVwrQJVcx5a4YESiCS");

#[program]
pub mod pigeon_battle {

    use super::*;

    pub fn mint_nft(ctx: Context<MintNft>, nft_class: u8) -> ProgramResult {
        let nft_info_account = &ctx.accounts.nft_info_account;

        let user_account = &ctx.accounts.user;
        let sol_price = nft_info_account.price; // Get the price from the PDA
        let user_balance = **user_account.try_borrow_lamports()?; // Check user's balance

        // Ensure the user has enough SOL
        if user_balance < sol_price {
            return Err(CustomError::InsufficientFunds.into());
        }

        // Transfer SOL from the user to the admin
        // `transfer`
        // - Accounts:
        //   - `[writable] from`
        //   - `[writable] to`
        // - Data:
        //   - lamports (u64)
        let admin_account = &ctx.accounts.admin;
        let transfer_instruction =
            system_instruction::transfer(&user_account.key(), &admin_account.key(), sol_price);

        invoke(
            &transfer_instruction,
            &[
                user_account.to_account_info(),
                admin_account.to_account_info(),
            ],
        )?;

        // Mint Token (NFT)
        let mint_account = &ctx.accounts.mint;
        let cpi_context = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::MintTo {
                mint: mint_account.to_account_info(),
                to: ctx.accounts.token.to_account_info(),
                authority: user_account.to_account_info(),
            },
        );
        token::mint_to(cpi_context, 1)?;

        // Initialize metadata for the NFT
        let cpi_context = CpiContext::new(
            ctx.accounts.token_metadata_program.to_account_info(),
            CreateMetadataAccountsV3 {
                metadata: ctx.accounts.metadata_account.to_account_info(),
                mint: mint_account.to_account_info(),
                mint_authority: user_account.to_account_info(),
                update_authority: user_account.to_account_info(),
                payer: user_account.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
            },
        );
        let data_v2 = DataV2 {
            name: "Pigeon Fight".to_string(),
            symbol: "PIGEON".to_string(),
            uri: "https://assets.pigeon-fight.xyz/sol-metadata/".to_string()
                + nft_class.to_string().as_str(),
            seller_fee_basis_points: 0,
            creators: None,
            collection: None,
            uses: None,
        };
        create_metadata_accounts_v3(cpi_context, data_v2, false, true, None)?;

        // Create master edition account
        let cpi_context = CpiContext::new(
            ctx.accounts.token_metadata_program.to_account_info(),
            CreateMasterEditionV3 {
                edition: ctx.accounts.master_edition_account.to_account_info(),
                mint: mint_account.to_account_info(),
                update_authority: user_account.to_account_info(),
                mint_authority: user_account.to_account_info(),
                payer: user_account.to_account_info(),
                metadata: ctx.accounts.metadata_account.to_account_info(),
                token_program: ctx.accounts.token_program.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
            },
        );
        create_master_edition_v3(cpi_context, None)?;

        // Create attribute account
        let attribute_account = &mut ctx.accounts.nft_attributes_account;
        attribute_account.class = nft_class;
        attribute_account.hp = 20;
        attribute_account.energy = 20;
        attribute_account.attack = 2 + nft_info_account.boost_attack;
        attribute_account.defense = 2 + nft_info_account.boost_defense;
        attribute_account.speed = 2 + nft_info_account.boost_speed;

        Ok(())
    }

    pub fn set_nft_data(
        ctx: Context<SetNftData>,
        _nft_class: u8,
        nft_data: NewNftClassData,
    ) -> ProgramResult {
        let nft_class_info_account = &mut ctx.accounts.nft_class_info;
        // Update NFT info
        nft_class_info_account.price = nft_data.price;
        nft_class_info_account.boost_attack = nft_data.boost_attack;
        nft_class_info_account.boost_defense = nft_data.boost_defense;
        nft_class_info_account.boost_speed = nft_data.boost_speed;
        Ok(())
    }

    pub fn set_item_data(
        ctx: Context<SetItemData>,
        _item_class: u8,
        item_data: NewItemClassData,
    ) -> ProgramResult {
        let item_class_info_account = &mut ctx.accounts.item_class_info;
        // Update Item Info
        item_class_info_account.price = item_data.price;
        item_class_info_account.health_hp = item_data.heal_hp;
        item_class_info_account.health_energy = item_data.heal_energy;
        item_class_info_account.boost_attack = item_data.boost_attack;
        item_class_info_account.boost_defense = item_data.boost_defense;
        item_class_info_account.boost_speed = item_data.boost_speed;
        Ok(())
    }

    pub fn upgrade_nft_attributes(
        ctx: Context<UpgradeNftAttributes>,
        points: AllocatedAttribute,
    ) -> ProgramResult {
        let nft_attribute_account = &mut ctx.accounts.nft_attribute_account;

        let total_point = exp_to_point(nft_attribute_account.exp);
        let remain_point = total_point - nft_attribute_account.allocated_point;
        let allocate_point =
            points.attack + points.defense + points.speed + points.max_hp + points.max_energy;
        if remain_point < allocate_point {
            return Err(CustomError::InvalidPoints.into());
        }

        // Update the attributes
        nft_attribute_account.attack += points.attack;
        nft_attribute_account.defense += points.defense;
        nft_attribute_account.speed += points.speed;
        nft_attribute_account.max_hp += points.max_hp;
        nft_attribute_account.max_energy += points.max_energy;

        // Evolute
        if nft_attribute_account.class <= 36 {
            // second evolution
            if nft_attribute_account.class <= 18 {
                if nft_attribute_account.allocated_point >= 310 {
                    nft_attribute_account.class *= 2;
                }
            }
            // first evolution
            if nft_attribute_account.class <= 10 {
                if nft_attribute_account.allocated_point >= 160 {
                    nft_attribute_account.class *= 2;
                }
            }
        }

        Ok(())
    }

    pub fn battle(ctx: Context<Battle>) -> ProgramResult {
        let my_attribute_account = &mut ctx.accounts.my_attributes;
        let op_attribute_account = &mut ctx.accounts.op_attributes;

        let result = internal_battle(my_attribute_account, op_attribute_account);
        msg!("Battle: {}", result);

        Ok(())
    }

    pub fn purchase_item(ctx: Context<PurchaseItem>, _item_class: u8) -> ProgramResult {
        let item_info = &ctx.accounts.item_info;

        let user_account = &ctx.accounts.user;
        let sol_price = item_info.price; // Get the price from the PDA
        let user_balance = **user_account.try_borrow_lamports()?; // Check user's balance

        // Ensure the user has enough SOL
        if user_balance < sol_price {
            return Err(CustomError::InsufficientFunds.into());
        }

        let nft_attributes_account = &mut ctx.accounts.nft_attributes_account;

        nft_attributes_account.hp += nft_attributes_account.max_hp * item_info.health_hp / 100;
        nft_attributes_account.energy +=
            nft_attributes_account.max_energy * item_info.health_energy / 100;

        nft_attributes_account.attack += item_info.boost_attack;
        nft_attributes_account.defense += item_info.boost_defense;
        nft_attributes_account.speed += item_info.boost_speed;

        Ok(())
    }
}
