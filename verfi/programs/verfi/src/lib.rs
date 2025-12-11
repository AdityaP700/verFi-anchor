use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");
//marks the module where the solana instructions live
#[program]
pub mod verfi {
    use super::*;

    pub fn create_event(
        ctx: Context<CreateEvent>,
        name: String,
    ) -> Result<()> {
        let event = &mut ctx.accounts.event;
        // We set the data on the blockchain
        event.authority = ctx.accounts.signer.key();
        event.name = name;
        event.bump = ctx.bumps.event;

        msg!("Event Created: {}", event.name);
        Ok(())
    }
    pub fn register_attendee(
        ctx: Context<RegisterAttendee>,
    ) -> Result<()>{
        let attendee_account= &mut ctx.accounts.attendee_account;
        let event = &ctx.accounts.event;
        let signer = &ctx.accounts.signer;
        attendee_account.event= event.key();
        attendee_account.attendee= signer.key();
        attendee_account.bump= ctx.bumps.attendee_account;
       msg!("Attendee Registered:{}",attendee_account.attendee);
       Ok(())
    }

}

#[derive(Accounts)]
#[instruction(name: String)]
pub struct CreateEvent<'info> {
    #[account(
        init,
        payer = signer,
        space = 8 + 32 + (4 + 50) + 1,
        seeds = [b"event", signer.key().as_ref(), name.as_bytes()],
        bump
    )]
    //seeds are like an unique desk in a conference hall with a predictable location because its arranged
    //basaed on organiser + event name
    //it refers to an account that our instruction needs access ti
    pub event : Account<'info,Event>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info,System>,
}

#[derive(Accounts)]
pub struct RegisterAttendee<'info>{
    #[account(mut)]
    pub signer: Signer<'info>,
    //it checks event accont first
    //as in does this PDA exists or not,does seed matches or bump matches??
    #[account(
        seeds=[b"event",event_authority.as_ref(),event_name.as_bytes()],
        bump = event.bump
    )]
 pub event : Account<'info,Event>,

    #[account(
        init,
        payer = signer,
        space = 8 + 32 + 32 + 1,
        //this badge belongs to a specific event
        //and a specific person
        seeds = [b"badge", event.key().as_ref(), signer.key().as_ref()],
        bump
    )]

    pub attendee_account: Account<'info,Attendee>,
    pub system_program: Program<'info,System>,

}

#[account]
pub struct Event {
    pub authority: Pubkey,
    pub name: String,
    pub bump: u8,
}

#[account]
pub struct Attendee {
    pub event: Pubkey,
    pub attendee: Pubkey,
    pub bump: u8,
}