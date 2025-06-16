use anchor_lang::prelude::*;

declare_id!("ESn9HUCzhjXYqSUpMDiSs1JV4znknv4LvHQFxUL3fTBG");

#[program]
pub mod sol_launch {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
