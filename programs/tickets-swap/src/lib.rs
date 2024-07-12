use anchor_lang::prelude::*;

declare_id!("9Pjw3S522y2xavPFvmZ1pSHU8akopTzXCdCY3iY9eRAA");

#[program]
pub mod tickets_swap {
    use super::*;

    // Initialise un nouvel événement
    pub fn initialize_event(
        ctx: Context<InitializeEvent>,
        name: String,
        description: String,
        start_date: i64,
        location: String,
        max_standard_tickets: u32,
        max_vip_tickets: u32,
        max_super_vip_tickets: u32,
    ) -> Result<()> {
        let event = &mut ctx.accounts.event;
        event.name = name;
        event.description = description;
        event.start_date = start_date;
        event.location = location;
        event.standard_tickets_available = max_standard_tickets;
        event.vip_tickets_available = max_vip_tickets;
        event.vvip_tickets_available = max_super_vip_tickets;
        event.sold_standard_tickets = 0;
        event.sold_vip_tickets = 0;
        event.sold_vvip_tickets = 0;
        event.creator = *ctx.accounts.creator.key;
        event.status = EventStatus::Active;
        Ok(())
    }

    // Modifie les détails d'un événement existant
    pub fn modify_event(
        ctx: Context<ModifyEvent>,
        name: Option<String>,
        description: Option<String>,
        start_date: Option<i64>,
        location: Option<String>,
    ) -> Result<()> {
        let event = &mut ctx.accounts.event;
        if let Some(name) = name {
            event.name = name;
        }
        if let Some(description) = description {
            event.description = description;
        }
        if let Some(start_date) = start_date {
            event.start_date = start_date;
        }
        if let Some(location) = location {
            event.location = location;
        }
        Ok(())
    }

    // Annule un événement existant
    pub fn cancel_event(ctx: Context<CancelEvent>) -> Result<()> {
        let event = &mut ctx.accounts.event;
        event.status = EventStatus::Cancelled;
        Ok(())
    }

    // Démarre un événement existant
    pub fn start_event(ctx: Context<StartEvent>) -> Result<()> {
        let event = &mut ctx.accounts.event;
        event.status = EventStatus::Started;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeEvent<'info> {
    #[account(init, payer = creator, space = 8 + 128 + 200 + 8 + 128 + 4 * 3 * 2 + 32 + 1, seeds = [b"event".as_ref(), creator.key().as_ref()], bump)]
    pub event: Account<'info, Event>,
    #[account(mut)]
    pub creator: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ModifyEvent<'info> {
    #[account(mut, has_one = creator, seeds = [b"event".as_ref(), creator.key().as_ref()], bump)]
    pub event: Account<'info, Event>,
    pub creator: Signer<'info>,
}

#[derive(Accounts)]
pub struct CancelEvent<'info> {
    #[account(mut, has_one = creator, seeds = [b"event".as_ref(), creator.key().as_ref()], bump)]
    pub event: Account<'info, Event>,
    pub creator: Signer<'info>,
}

#[derive(Accounts)]
pub struct StartEvent<'info> {
    #[account(mut, has_one = creator, seeds = [b"event".as_ref(), creator.key().as_ref()], bump)]
    pub event: Account<'info, Event>,
    pub creator: Signer<'info>,
}

#[account]
pub struct Event {
    pub name: String,
    pub description: String,
    pub start_date: i64,
    pub location: String,
    pub standard_tickets_available: u32,
    pub sold_standard_tickets: u32,
    pub vip_tickets_available: u32,
    pub sold_vip_tickets: u32,
    pub vvip_tickets_available: u32,
    pub sold_vvip_tickets: u32,
    pub creator: Pubkey,
    pub status: EventStatus,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum EventStatus {
    Active,
    Cancelled,
    Started,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum TicketType {
    Standard,
    Vip,
    VeryVip,
}

#[error_code]
pub enum CustomError {
    #[msg("The event is not active.")]
    EventNotActive,
    #[msg("The event is sold out.")]
    EventSoldOut,
    #[msg("The event has already started.")]
    EventAlreadyStarted,
    #[msg("No tickets available to refund.")]
    NoTicketsToRefund,
}
