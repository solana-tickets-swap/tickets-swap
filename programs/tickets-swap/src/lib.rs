#![allow(clippy::result_large_err)]

use {
    anchor_lang::prelude::*,
    anchor_spl::{
        associated_token::AssociatedToken,
        metadata::{
            create_master_edition_v3, create_metadata_accounts_v3,
            mpl_token_metadata::types::DataV2, CreateMasterEditionV3, CreateMetadataAccountsV3,
            Metadata,
        },
        token::{mint_to, Mint, MintTo, Token, TokenAccount},
    },
};
declare_id!("9Pjw3S522y2xavPFvmZ1pSHU8akopTzXCdCY3iY9eRAA");

#[program]
mod tickets_swap {
    use super::*;
    pub fn create_event(
        ctx: Context<CreateEvent>,
        title: String,
        description: String,
        location: String,
        date: u64,
    ) -> Result<()> {
        let event = &mut ctx.accounts.event;
        event.owner = *ctx.accounts.owner.key;
        event.title = title;
        event.description = description;
        event.location = location;
        event.date = date;
        event.tickets_available = 0;
        event.tickets_sold = 0;
        event.ticket_price = 0;
        event.status = EventStatus::Pending;

        msg!("Event {} created by {}", event.title, event.owner);

        Ok(())
    }

    pub fn mint_tickets(
        ctx: Context<MintTickets>,
        ticket_symbol: String,
        ticket_uri: String,
        ticket_quantity: u64,
        ticket_price: u16,
    ) -> Result<()> {
        let event = &mut ctx.accounts.event;

        msg!("Minting tickets");

        let cpi_context = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint: ctx.accounts.mint_account.to_account_info(),
                to: ctx.accounts.associated_token_account.to_account_info(),
                authority: ctx.accounts.payer.to_account_info(),
            },
        );

        mint_to(cpi_context, ticket_quantity)?;

        msg!("Creating metadata account");

        let cpi_context = CpiContext::new(
            ctx.accounts.token_metadata_program.to_account_info(),
            CreateMetadataAccountsV3 {
                metadata: ctx.accounts.metadata_account.to_account_info(),
                mint: ctx.accounts.mint_account.to_account_info(),
                mint_authority: ctx.accounts.payer.to_account_info(),
                update_authority: ctx.accounts.payer.to_account_info(),
                payer: ctx.accounts.payer.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
            },
        );

        create_metadata_accounts_v3(
            cpi_context,
            DataV2 {
                name: event.title.clone(),
                symbol: ticket_symbol,
                uri: ticket_uri,
                seller_fee_basis_points: 0,
                creators: None,
                collection: None,
                uses: None,
            },
            false, // is mutable ?
            true,  // update_authority is signer
            None,  // Collections details
        )?;

        msg!("Creating master edition");
        let cpi_context = CpiContext::new(
            ctx.accounts.token_metadata_program.to_account_info(),
            CreateMasterEditionV3 {
                edition: ctx.accounts.edition_account.to_account_info(),
                mint: ctx.accounts.mint_account.to_account_info(),
                update_authority: ctx.accounts.payer.to_account_info(),
                mint_authority: ctx.accounts.payer.to_account_info(),
                payer: ctx.accounts.payer.to_account_info(),
                metadata: ctx.accounts.metadata_account.to_account_info(),
                token_program: ctx.accounts.token_program.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
            },
        );
        create_master_edition_v3(cpi_context, Some(ticket_quantity))?;

        event.tickets_available = ticket_quantity;
        event.ticket_price = ticket_price;
        event.status = EventStatus::Active;

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(title: String)]
pub struct CreateEvent<'info> {
    #[account(
        init,
        seeds = [title.as_bytes(), owner.key().as_ref()],
        bump,
        payer = owner,
        space = 8 + 32 + 54 + 204 + 54 + 8 + 2 + 2 + 2 + 1,
    )]
    pub event: Account<'info, Event>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct MintTickets<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut)]
    pub event: Account<'info, Event>,
    /// CHECK: Validate address by deriving pda
    #[account(
        mut,
        seeds = [b"metadata", token_metadata_program.key().as_ref(), mint_account.key().as_ref()],
        bump,
        seeds::program = token_metadata_program.key(),
    )]
    pub metadata_account: UncheckedAccount<'info>,

    /// CHECK: Validate address by deriving pda
    #[account(
        mut,
        seeds = [b"metadata", token_metadata_program.key().as_ref(), mint_account.key().as_ref(), b"edition"],
        bump,
        seeds::program = token_metadata_program.key(),
    )]
    pub edition_account: UncheckedAccount<'info>,
    #[account(
        init,
        payer = payer,
        mint::decimals = 0,
        mint::authority = payer.key(),
        mint::freeze_authority = payer.key(),
    )]
    pub mint_account: Account<'info, Mint>,
    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = mint_account,
        associated_token::authority = payer,
    )]
    pub associated_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub token_metadata_program: Program<'info, Metadata>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[account]
pub struct Event {
    pub owner: Pubkey,
    pub title: String,
    pub description: String,
    pub location: String,
    pub date: u64,
    pub tickets_available: u64,
    pub tickets_sold: u16,
    pub ticket_price: u16,
    pub status: EventStatus,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum EventStatus {
    Pending,
    Active,
    Cancelled,
}
