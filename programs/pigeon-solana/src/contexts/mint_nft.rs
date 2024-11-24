use anchor_lang::{
    prelude::*,
    solana_program::{program::invoke, system_instruction},
};
use anchor_spl::metadata::mpl_token_metadata::{
    instructions::{
        CreateMasterEditionV3Cpi, CreateMasterEditionV3CpiAccounts,
        CreateMasterEditionV3InstructionArgs, CreateMetadataAccountV3Cpi,
        CreateMetadataAccountV3CpiAccounts, CreateMetadataAccountV3InstructionArgs,
    },
    types::{Collection, Creator, DataV2},
};
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::Metadata,
    token::{mint_to, Mint, MintTo, Token, TokenAccount},
};

use crate::{
    account::{NftAttributes, PigeonConfig},
    error::CustomError,
};

use super::NftClassInfo;

#[derive(Accounts)]
#[instruction(nft_class: u8)]
pub struct MintNFT<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    /// CHECK: ADMIN to receive fund - fixed address
    #[account(
        mut,
        address = pigeon_config.admin_key @ CustomError::Unauthorized
    )]
    pub admin: AccountInfo<'info>,
    #[account(
        seeds = [b"pigeon_config"],
        bump,
    )]
    pub pigeon_config: Account<'info, PigeonConfig>,
    #[account(
        seeds = [b"nft_data", nft_class.to_le_bytes().as_ref()],
        bump,
    )]
    pub nft_info_account: Account<'info, NftClassInfo>,
    #[account(
        init,
        payer = owner,
        mint::decimals = 0,
        mint::authority = mint_authority,
        mint::freeze_authority = mint_authority,
    )]
    pub mint: Account<'info, Mint>,
    #[account(
        init,
        payer = owner,
        associated_token::mint = mint,
        associated_token::authority = owner
    )]
    pub destination: Account<'info, TokenAccount>,
    #[account(mut)]
    /// CHECK: This account will be initialized by the metaplex program
    pub metadata: UncheckedAccount<'info>,
    #[account(mut)]
    /// CHECK: This account will be initialized by the metaplex program
    pub master_edition: UncheckedAccount<'info>,
    #[account(
        seeds = [b"authority"],
        bump,
    )]
    /// CHECK: This is account is not initialized and is being used for signing purposes only
    pub mint_authority: UncheckedAccount<'info>,
    #[account(mut)]
    pub collection_mint: Account<'info, Mint>,
    #[account(
        init,
        payer = owner,
        seeds = [b"attributes", destination.key().as_ref()],
        bump,
        space = 27 // u8 (1 byte) + u16 (2 bytes) * 9 => 8 (discriminator) + 19 (data)
    )]
    pub nft_attributes_account: Account<'info, NftAttributes>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_metadata_program: Program<'info, Metadata>,
    pub rent: Sysvar<'info, Rent>,
}

impl<'info> MintNFT<'info> {
    pub fn mint_nft(&mut self, bumps: &MintNFTBumps, nft_class: u8) -> Result<()> {
        let nft_info_account = &self.nft_info_account;
        let admin = &self.admin;
        let metadata = &self.metadata.to_account_info();
        let master_edition = &self.master_edition.to_account_info();
        let mint = &self.mint.to_account_info();
        let authority = &self.mint_authority.to_account_info();
        let payer = &self.owner.to_account_info();
        let system_program = &self.system_program.to_account_info();
        let spl_token_program = &self.token_program.to_account_info();
        let spl_metadata_program = &self.token_metadata_program.to_account_info();
        let attribute_account = &mut self.nft_attributes_account;

        let sol_price = nft_info_account.price; // Get the price from the PDA
        let user_balance = **payer.try_borrow_lamports()?; // Check user's balance

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
        let transfer_instruction =
            system_instruction::transfer(&payer.key(), &admin.key(), sol_price);

        invoke(
            &transfer_instruction,
            &[payer.to_account_info(), admin.to_account_info()],
        )?;

        // Mint NFT
        let seeds = &[&b"authority"[..], &[bumps.mint_authority]];
        let signer_seeds = &[&seeds[..]];

        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = MintTo {
            mint: self.mint.to_account_info(),
            to: self.destination.to_account_info(),
            authority: self.mint_authority.to_account_info(),
        };
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);
        mint_to(cpi_ctx, 1)?;
        msg!("NFT minted!");

        let creator = vec![Creator {
            address: self.mint_authority.key(),
            verified: false,
            share: 100,
        }];

        let metadata_account = CreateMetadataAccountV3Cpi::new(
            spl_metadata_program,
            CreateMetadataAccountV3CpiAccounts {
                metadata,
                mint,
                mint_authority: authority,
                payer,
                update_authority: (authority, true),
                system_program,
                rent: None,
            },
            CreateMetadataAccountV3InstructionArgs {
                data: DataV2 {
                    name: "Pigeon Fight".to_string(),
                    symbol: "PIGEON".to_string(),
                    uri: "https://assets.pigeon-fight.xyz/sol-metadata/".to_string()
                        + nft_class.to_string().as_str(),
                    seller_fee_basis_points: 0,
                    creators: Some(creator),
                    collection: Some(Collection {
                        verified: true,
                        key: self.collection_mint.key(),
                    }),
                    uses: None,
                },
                is_mutable: true,
                collection_details: None,
            },
        );
        metadata_account.invoke_signed(signer_seeds)?;

        let master_edition_account = CreateMasterEditionV3Cpi::new(
            spl_metadata_program,
            CreateMasterEditionV3CpiAccounts {
                edition: master_edition,
                update_authority: authority,
                mint_authority: authority,
                mint,
                payer,
                metadata,
                token_program: spl_token_program,
                system_program,
                rent: None,
            },
            CreateMasterEditionV3InstructionArgs {
                max_supply: Some(0),
            },
        );
        master_edition_account.invoke_signed(signer_seeds)?;

        // Create attribute account
        attribute_account.class = nft_class;
        attribute_account.hp = 20;
        attribute_account.energy = 20;
        attribute_account.attack = 2 + nft_info_account.boost_attack;
        attribute_account.defense = 2 + nft_info_account.boost_defense;
        attribute_account.speed = 2 + nft_info_account.boost_speed;

        Ok(())
    }
}
