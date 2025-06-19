use anchor_lang::prelude::*;

declare_id!("5HPgLUD3cPwHJ8FmTLXM5nkxnqQTceMebefLzHZXqr2x");

#[program]
pub mod pdacreator {
    use super::*;

    pub fn create_pda(ctx: Context<CreatePda>) -> Result<()> {
        let pda_account = &mut ctx.accounts.pda_account;
        pda_account.authority = ctx.accounts.authority.key();
        pda_account.number = 25;
        pda_account.bump = ctx.bumps.pda_account;
        
        msg!("PDA account created with number: {}", pda_account.number);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreatePda<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    
    #[account(
        init,
        payer = authority,
        space = 8 + 32 + 8 + 1, // discriminator + pubkey + number + bump
        seeds = [b"pda-number", authority.key().as_ref()],
        bump
    )]
    pub pda_account: Account<'info, PdaAccount>,
    
    pub system_program: Program<'info, System>,
}

#[account]
pub struct PdaAccount {
    pub authority: Pubkey,    // 32 bytes
    pub number: u64,         // 8 bytes
    pub bump: u8,           // 1 byte
}