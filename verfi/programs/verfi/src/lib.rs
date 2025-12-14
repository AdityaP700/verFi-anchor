use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, Mint, TokenAccount, MintTo};
use anchor_spl::metadata::{
    create_metadata_accounts_v3,
    CreateMetadataAccountsV3,
    Metadata as Metaplex,
};
use mpl_token_metadata::types::DataV2;
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
        event.total_minted = 0;

        msg!("Event Created: {}", event.name);
        Ok(())
    }
    pub fn register_attendee(
        ctx: Context<RegisterAttendee>,
    ) -> Result<()>{
        let attendee_account= &mut ctx.accounts.attendee_account;
        let event = &mut ctx.accounts.event;
        let signer = &ctx.accounts.signer;
        let metadata_account = &ctx.accounts.metadata_account;
        
        attendee_account.event= event.key();
        attendee_account.attendee= signer.key();
        attendee_account.bump= ctx.bumps.attendee_account;
        event.total_minted +=1;

        let authority_key = event.authority.key();
        let name_bytes = event.name.as_bytes();
        let event_bump = event.bump;
        let seeds = &[b"event", authority_key.as_ref(), name_bytes, &[event_bump]];
        let signer_seeds n= &[&seeds[..]];

        let mint_ctx=CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            //anchor internally recreates the seeeds using to_account_info()
            MintTo{
                mint: ctx.accounts.mint.to_account_info(),
                to: ctx.accounts.token_account.to_account_info(),
                authority: event.to_account_info(),//the event itself
            },
            signer_seeds,//Proof we are the event
        );
        token:mint_to(mint_ctx,1)?;

        let metadata_ctx = CpiContext::new_with_signer(
            ctx.accounts.metadata_program.to_account_info(),
            CreateMetadataAccountsV3 {
                metadata: ctx.accounts.metadata_account.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
                mint_authority: event.to_account_info(),
                payer: ctx.accounts.signer.to_account_info(),
                update_authority: event.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
            },
            signer_seeds
        );
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
        mut,
        seeds=[b"event",event.authority.as_ref(),event.name.as_bytes()],
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
        bump,
        mint::decimals=0,
        mint::authority=event,
        mint::freeze_authority=event,
    )]
    pub mint: Account<'info,Mint>,
    #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint=mint,
        associated_token::authority=signer,
    )]
pub token_account: Account<'info,TokenAccount>,

    pub attendee_account: Account<'info,Attendee>,
    pub system_program: Program<'info,System>,
    pub token_program: Program<'info,Token>,
    pub rent: Sysvar<'info,Rent>,
    pub associated_token_program: Program<'info,anchor_spl::associated_token::AssociatedToken>,
}

#[account]
pub struct Event {
    pub authority: Pubkey,
    pub name: String,
    pub bump: u8,
    pub total_minted : u64,
}

#[account]
pub struct Attendee {
    pub event: Pubkey,
    pub attendee: Pubkey,
    pub bump: u8,
}