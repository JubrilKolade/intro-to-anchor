// Import anchor
use anchor_lang::prelude::*;

declare_id!("AFPP1qh8mmEVasachEnsDnRwpTATB4J6XTYWLeJBzuZK");

#[program]
mod hello_world {
    use super::*;

    pub fn hello(ctx: Context<Hello>) -> Result<()> {
        msg!("Hello, World!");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Hello {}