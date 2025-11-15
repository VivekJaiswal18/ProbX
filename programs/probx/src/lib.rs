use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Mint, MintTo};
declare_id!("AqYPpFiHgjEpabs9iFGBpbuazffLeVYTDU3yHyvxxivm");

#[program]
pub mod probx{
    use super::*;

    pub fn initialize(ctx: Context<Initialize>)->Result<()>{
        let market = &mut ctx.accounts.market;

        market.authority = ctx.accounts.payer.key();
        market.collateral_mint = ctx.accounts.collateral_mint.key();
        market.yes_mint = ctx.accounts.yes_mint.key();
        market.no_mint = ctx.accounts.no_mint.key();
        market.yes_reserve = 0;
        market.no_reserve = 0;
        market.collateral_reserve = 0;
        market.fee_bps = 0;
        market.resolved = false;
        market.winning_outcome = 0;
        market.bump = ctx.bumps.market;

        // let yes_mint = ctx.accounts.yes_mint.key().as_ref();
        // print!("yes_mint address {}", yes_mint);
        // let no_mint = ctx.accounts.yes_mint.key().as_ref();
        // print!("no_mint address {}", no_mint);
        // let collateral_mint = ctx.accounts.yes_mint.key().as_ref();
        // print!("collateral_mint address {}", collateral_mint);
        Ok(())
}

    pub fn add_liquidity(ctx: Context<AddLiquidity>)->Result<()>{
        
    }
}

#[derive(Accounts)]
pub struct Initialize<'info>{
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(init, payer = payer, seeds = [b"market", payer.key().as_ref()], bump, space= 8+32+32+32+32+8+8+8+2+1+1+1)]
    pub market: Account<'info, Market>,
    pub collateral_mint: Account<'info, Mint>,
    #[account(init, payer = payer, seeds = [b"yes_mint", market.key().as_ref()], bump, mint::decimals = 0, mint::authority = market)]
    pub yes_mint: Account<'info, Mint>,
    #[account(init, payer = payer, seeds = [b"no_mint", market.key().as_ref()], bump, mint::decimals = 0, mint::authority = market)]
    pub no_mint: Account<'info, Mint>,
    #[account(init, payer = payer, seeds = [b"collateral_reserve", market.key().as_ref()], bump, space = 8+165)]
    pub collateral_reserve: Account<'info, TokenAccount>,
    #[account(init, payer = payer, seeds = [b"yes_reserve", market.key().as_ref()], bump, space = 8+165)]
    pub yes_reserve: Account<'info, TokenAccount>,
    #[account(init, payer = payer, seeds = [b"no_reserve", market.key().as_ref()], bump, space = 8+165)]
    pub no_reserve: Account<'info, TokenAccount>,
    pub system_program : Program<'info, System>,
    pub token_program : Program<'info, Token>,
    pub associated_token_program: Program<'info, anchor_spl::associated_token::AssociatedToken>,
}

#[derive(Accounts)]
pub struct AddLiquidity<'info>{
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut, seeds = [b"market", market.authority.as_ref()], bump = market.bump)]
    pub market: Account<'info, Market>,
    #[account(mut)]
    pub yes_mint: Account<'info, Mint>,
    #[account(mut)]
    pub no_mint: Account<'info, Mint>,
    #[account(mut, seeds = [b"yes_reserve", market.key.as_ref()], bump = market.bump)]
    pub yes_reserve: Account<'info, TokenAccount>,
    #[account(mut, seeds = [b"no_reserve", market.key.as_ref()], bump = market.bump)]
    pub no_reserve: Account<'info, TokenAccount>,
    #[account(mut, seeds = [b"collateral_reserve", market.key.as_ref()], bump = market.bump)]
    pub collateral_reserve: Account<'info, TokenAccount>,
    pub collateral_mint: Account<'info, Mint>,

}

#[account]
pub struct Market {
    pub authority: Pubkey,           // creator / resolver
    pub collateral_mint: Pubkey,     // e.g., USDC mint
    pub yes_mint: Pubkey,            // YES token mint
    pub no_mint: Pubkey,             // NO token mint
    pub yes_reserve: u64,            // YES pool reserve
    pub no_reserve: u64,             // NO pool reserve
    pub collateral_reserve: u64,       // collateral backing
    pub fee_bps: u16,                // trading fee %
    pub resolved: bool,              // if oracle resolved
    pub winning_outcome: u8,         // 0=unknown, 1=YES, 2=NO
    pub bump: u8,
}


#[derive(Accounts)]
pub struct Buy<'info>{
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut, seeds = [b"market", market.authority.as_ref()], bump = market.bump)]
    pub market: Account<'info, Market>,
    #[account(mut, seeds = [b"yes_reserve", market.key.as_ref()], bump = market.bump)]
    pub yes_reserve: Account<'info, TokenAccount>,
    #[account(mut, seeds = [b"collateral_reserve", market.key.as_ref()], bump = market.bump)]
    pub collateral_reserve: Account<'info, TokenAccount>,
    #[account(mut, seeds = [b"no_reserve", market.key().as_ref()], bump = market.bump)]
    pub no_reserve : Account<'info, TokenAccount>,
    pub system_program : Program<'info, System>,
    pub token_program : Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

#[derive(Accounts)]
pub struct Sell<'info>{
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut, seeds = [b"market", market.authority.as_ref()], bump = market.bump)]
    pub market: Account<'info, Market>,
    #[account(mut, seeds = [b"collateral_reserve", market.key.as_ref()], bump = market.bump)]
    pub collateral_reserve: Account<'info, TokenAccount>,
    #[account(mut, seeds = [b"yes_reserve", market.key.as_ref()], bump = market.bump)]
    pub yes_reserve: Account<'info, TokenAccount>,
    #[account(mut, seeds = [b"no_reserve", market.key.as_ref()], bump = market.bump)]
    pub no_reserve: Account<'info, TokenAccount>,
    pub system_program : Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}


