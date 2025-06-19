use anchor_lang::prelude::*;

declare_id!("DzUEHk2zpTwMgZkMpLQczo8XiprTrWmbi196ij8RMH5Z");

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
