use anchor_lang::prelude::*;

pub mod constants;
pub mod errors;
pub mod instructions;
pub mod states;
pub mod utils;

use instructions::*;

declare_id!("6NchgAdLqvbXafnsHobsvPsaepA9QDh5ccsWVcbBRojx");
#[program]
pub mod baked_beans {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, new_authority: Pubkey) -> Result<()> {
        initialize_handle(ctx, new_authority)
    }

    pub fn buy_roogs(ctx: Context<BuyRoogs>, amount: u64) -> Result<()> {
        buy_roogs_handle(ctx, amount)
    }

    pub fn sell_roogs(ctx: Context<SellRoogs>) -> Result<()> {
        sell_roogs_handle(ctx)
    }

    pub fn hatch_roogs(ctx: Context<HatchRoogs>) -> Result<()> {
        hatch_roogs_handle(ctx)
    }
}
