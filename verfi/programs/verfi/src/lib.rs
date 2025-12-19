use anchor_lang::prelude::*;
use anchor_spl::{
    token::{self, Mint, MintTo, Token, TokenAccount},
    associated_token::AssociatedToken,
};
use anchor_spl::metadata::{
    create_metadata_accounts_v3,
    CreateMetadataAccountsV3,
    Metadata as Metaplex,
};
use mpl_token_metadata::types::DataV2;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod verfi {
    use super::*;

    pub fn create_event(
        ctx: Context<CreateEvent>,
        name: String,
        uri: String,
    ) -> Result<()> {
        let event = &mut ctx.accounts.event;
        event.authority = ctx.accounts.signer.key();
        event.name = name;
        event.uri = uri;
        event.bump = ctx.bumps.event;
        event.total_minted = 0;

        msg!("Event Created: {}", event.name);
        Ok(())
    }

    pub fn register_attendee(ctx: Context<RegisterAttendee>) -> Result<()> {
        let event = &mut ctx.accounts.event;
        let attendee = &mut ctx.accounts.attendee_account;

        attendee.event = event.key();
        attendee.attendee = ctx.accounts.signer.key();
        attendee.bump = ctx.bumps.attendee_account;

        event.total_minted += 1;

        let seeds = &[
            b"event",
            event.authority.as_ref(),
            event.name.as_bytes(),
            &[event.bump],
        ];
        let signer_seeds = &[&seeds[..]];

        // Mint POAP token
        let mint_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint: ctx.accounts.mint.to_account_info(),
                to: ctx.accounts.token_account.to_account_info(),
                authority: event.to_account_info(),
            },
            signer_seeds,
        );
        token::mint_to(mint_ctx, 1)?;

        // Metadata
        let metadata = DataV2 {
            name: event.name.clone(),
            symbol: "POAP".to_string(),
            uri: event.uri.clone(),
            seller_fee_basis_points: 0,
            creators: None,
            collection: None,
            uses: None,
        };

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
            signer_seeds,
        );

        create_metadata_accounts_v3(
            metadata_ctx,
            metadata,
            false,
            true,
            None,
        )?;

        msg!("Attendee Registered: {}", attendee.attendee);
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(name: String, uri: String)]
pub struct CreateEvent<'info> {
    #[account(
        init,
        payer = signer,
        space = 8 + 32 + 4 + name.len() + 4 + uri.len() + 1 + 8,
        seeds = [b"event", signer.key().as_ref(), name.as_bytes()],
        bump
    )]
    pub event: Account<'info, Event>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RegisterAttendee<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds = [b"event", event.authority.as_ref(), event.name.as_bytes()],
        bump = event.bump
    )]
    pub event: Account<'info, Event>,

    #[account(
        init,
        payer = signer,
        space = 8 + 32 + 32 + 1,
        seeds = [b"badge", event.key().as_ref(), signer.key().as_ref()],
        bump
    )]
    pub attendee_account: Account<'info, Attendee>,

    #[account(
        init,
        payer = signer,
        seeds = [b"mint", event.key().as_ref(), signer.key().as_ref()],
        bump,
        mint::decimals = 0,
        mint::authority = event,
        mint::freeze_authority = event
    )]
    pub mint: Account<'info, Mint>,

    #[account(
        init,
        payer = signer,
        associated_token::mint = mint,
        associated_token::authority = signer
    )]
    pub token_account: Account<'info, TokenAccount>,

    /// CHECK: Metaplex PDA
    #[account(
        mut,
        seeds = [
            b"metadata",
            metadata_program.key().as_ref(),
            mint.key().as_ref()
        ],
        bump,
        seeds::program = metadata_program.key()
    )]
    pub metadata_account: UncheckedAccount<'info>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
    pub metadata_program: Program<'info, Metaplex>,
}

#[account]
pub struct Event {
    pub authority: Pubkey,
    pub name: String,
    pub uri: String,
    pub bump: u8,
    pub total_minted: u64,
}

#[account]
pub struct Attendee {
    pub event: Pubkey,
    pub attendee: Pubkey,
    pub bump: u8,
}
