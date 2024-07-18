use anchor_lang::prelude::*;

// Déclare l'ID du programme
declare_id!("FDpDx1vfXUn9FNPWip6VVr2HrUC5Mq6Lb6P73rQPtQMa");

#[program]
pub mod tickets_swap {
    use super::*;

    // Instruction permettant de créer un événement
    pub fn create_event(
        ctx: Context<CreateEvent>,
        title: String,
        description: String,
        date: i64,
        location: String,
        ticket_price: u64,
    ) -> Result<()> {
        let event = &mut ctx.accounts.event; // Accède au compte de l'événement
        event.title = title;
        event.description = description;
        event.date = date;
        event.location = location;
        event.organizer = *ctx.accounts.organizer.key; // Définit l'organisateur de l'événement
        event.ticket_price = ticket_price;
        Ok(())
    }

    // Instruction permettant de créer un ticker pour un X événement
    pub fn buy_ticket(ctx: Context<BuyTicket>, date_of_purchase: i64) -> Result<()> {
        let ticket = &mut ctx.accounts.ticket;
        let event = &ctx.accounts.event;

        ticket.event = event.key();
        ticket.price = event.ticket_price; // Attribuer le prix du billet de l'événement.
        ticket.date_of_purchase = date_of_purchase; // Prix de quand le owner a acheté ce ticket (on conserve un log au cas où le prix joint à l'event change)
        ticket.owner = *ctx.accounts.owner.key; // Définit le créateur (proprio) du ticket
        Ok(())
    }
}

// Contexte de l'instruction permettant de créer un événement
#[derive(Accounts)]
pub struct CreateEvent<'info> {
    // Initialise le compte de l'événement, en spécifiant le payeur et l'espace nécessaire
    #[account(init, payer = organizer, space = 8 + 32 + 4 + 100 + 4 + 256 + 8 + 4 + 100 + 8)]
    pub event: Account<'info, Event>,
    #[account(mut)]
    pub organizer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// Contexte de l'instruction permettant de créer un ticker pour un X événement
#[derive(Accounts)]
pub struct BuyTicket<'info> {
    // Initialise le compte du ticket, en spécifiant le payeur et l'espace nécessaire
    #[account(init, payer = owner, space = 8 + 32 + 32 + 8 + 8)]
    pub ticket: Account<'info, Ticket>,
    pub event: Account<'info, Event>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// Structure pour stocker les informations de l'événement
#[account]
pub struct Event {
    pub title: String,
    pub description: String,
    pub date: i64,
    pub location: String,
    pub ticket_price: u64,
    pub organizer: Pubkey, // Clé publique de l'organisateur de l'événement
}

#[account]
pub struct Ticket {
    pub event: Pubkey,
    pub price: u64,
    pub date_of_purchase: i64,
    pub owner: Pubkey,
}
