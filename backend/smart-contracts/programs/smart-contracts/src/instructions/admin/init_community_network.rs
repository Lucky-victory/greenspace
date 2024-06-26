use crate::{errors::ErrorCodes, state::*};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};
use crate::constants::*;

#[derive(Accounts)]
pub struct InitCommunityNetwork<'info> {
    #[account(
        init,
        seeds = [b"community-network"/*, authority.key().as_ref()*/],
        bump,
        payer = authority, 
        space = CommunityNetwork::LEN
    )]

    pub community_network: Box<Account<'info, CommunityNetwork>>,

    #[account(
        mut,
        seeds = ["user-mint".as_bytes().as_ref()],
        bump
    )]

    pub user_nft_mint: Account<'info, Mint>,

    #[account(
        mut,
        seeds = ["nutritionist-mint".as_bytes().as_ref()],
        bump
    )]

    pub nutritionist_nft_mint: Account<'info, Mint>,

    #[account(
        //init,
        //payer = authority,
         //token::mint = USDC_MINT_PUBKEY,
         //token::authority = authority.key(),
        associated_token::mint = USDC_MINT_PUBKEY,
        associated_token::authority = authority.key()
      )]
    pub community_network_vault_usdc_account: Account<'info, TokenAccount>,

    #[account(mut, address = ADMIN)]
    pub authority: Signer<'info>,

    pub token_program: Program<'info, Token>,

    pub associated_token_program: Program<'info, AssociatedToken>,

    pub system_program: Program<'info, System>,

    pub rent: Sysvar<'info, Rent>

}

pub fn init_community_network_handler(ctx: Context<InitCommunityNetwork>) -> Result<()> {
    let community_network = &mut ctx.accounts.community_network;

    community_network.admin = ctx.accounts.authority.key();
    community_network.community_network_usdc_vault = ctx.accounts.community_network_vault_usdc_account.key();
    community_network.user_nft_mint = ctx.accounts.user_nft_mint.key();
    community_network.nutritionist_nft_mint = ctx.accounts.nutritionist_nft_mint.key();
    community_network.total_nutritionist_applications = 0;
    community_network.total_whitelisted_nutritionists = 0;
    community_network.total_users = 0;
    community_network.bump = *ctx.bumps.get("community_network").unwrap();

    Ok(())
}