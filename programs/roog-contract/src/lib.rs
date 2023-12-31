use anchor_lang::prelude::*;

declare_id!("6NchgAdLqvbXafnsHobsvPsaepA9QDh5ccsWVcbBRojx");

#[program]
pub mod roog_contract {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
