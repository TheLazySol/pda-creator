use anchor_lang::prelude::*;

declare_id!("5HPgLUD3cPwHJ8FmTLXM5nkxnqQTceMebefLzHZXqr2x");

#[program]
pub mod pdacreator {
    use super::*;

    pub fn greet(_ctx: Context<Initialize>) -> Result<()> {
        msg!("GM!");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[account]
pub struct PdaCreator {
    pub authority: Pubkey,
    pub pda: Pubkey,
    pub bump: u8,
}

#[derive(Accounts)]
pub struct CreatePda {}