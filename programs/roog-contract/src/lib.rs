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

    pub fn buy_eggs(ctx: Context<BuyEggs>, amount: u64) -> Result<()> {
        buy_eggs_handle(ctx, amount)
    }

    pub fn sell_eggs(ctx: Context<SellEggs>) -> Result<()> {
        sell_eggs_handle(ctx)
    }

    pub fn hatch_eggs(ctx: Context<HatchEggs>) -> Result<()> {
        hatch_eggs_handle(ctx)
    }
}
