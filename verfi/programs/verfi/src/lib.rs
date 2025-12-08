use anchor_lang::prelude::*;

// 1. This is the Program ID. Anchor generates this for you.
// If your terminal says "ID doesn't match", we will fix it later.
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod verfi {
    use super::*;

    // 2. THE INSTRUCTION ( The Function )
    // This is what the Frontend calls. "Hey, create an event!"
    pub fn create_event(
        ctx: Context<CreateEvent>,
        name: String,
    ) -> Result<()> {
        // We get the event account from the Context (security check passed!)
        let event = &mut ctx.accounts.event;

        // We set the data on the blockchain
        event.authority = ctx.accounts.signer.key(); // Who owns this event?
        event.name = name;                           // What is the event name?
        event.bump = ctx.bumps.event;                // Store the 'bump' (explained below)

        msg!("Event Created: {}", event.name);
        Ok(())
    }
}

// 3. THE DATA STRUCTURE ( The Shape of the File )
// This defines what "Event" actually looks like on the hard drive (blockchain).
#[account]
pub struct Event {
    pub authority: Pubkey,  // 32 bytes: The address of the organizer
    pub name: String,       // 4 + len bytes: The name of the event
    pub bump: u8,           // 1 byte: A math helper for the address
}

// 4. THE CONTEXT ( The Bouncer / Security )
// Before the function runs, Anchor checks all these rules.
#[derive(Accounts)]
#[instruction(name: String)] // We need the 'name' here to calculate the address
pub struct CreateEvent<'info> {

    // THE MAGIC: PDA (Program Derived Address)
    // We want the Event Address to be easy to find.
    // Instead of a random address, we calculate it using:
    // "event" + organizer_address + event_name
    #[account(
        init,                         // Action: Initialize (create) a new account
        payer = signer,               // Who pays the rent? The signer.
        space = 8 + 32 + (4 + 50) + 1, // How big is the file? (Rent calculation)
        seeds = [b"event", signer.key().as_ref(), name.as_bytes()],
        bump                          // Calculate the "bump" automatically
    )]
    pub event: Account<'info, Event>,

    #[account(mut)]
    pub signer: Signer<'info>, // The person clicking the button

    pub system_program: Program<'info, System>, // Required to create accounts
}