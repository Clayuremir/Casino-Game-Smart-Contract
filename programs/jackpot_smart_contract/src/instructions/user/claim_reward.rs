//! Instruction for winners to claim their rewards.

use crate::{
    constants::{CONFIG, GAME_GROUND, GLOBAL},
    errors::*,
    state::{config::*, gameground::*},
    utils::*,
};
use anchor_lang::{prelude::*, system_program};

/// Accounts required for claiming a reward.
#[derive(Accounts)]
#[instruction(round_num: u64)]
pub struct ClaimReward<'info> {
    #[account(
        mut,
        seeds = [CONFIG.as_bytes()],
        bump,
    )]
    global_config: Box<Account<'info, Config>>,

    /// CHECK: global vault pda which stores SOL
    #[account(
        mut,
        seeds = [GLOBAL.as_bytes()],
        bump,
    )]
    pub global_vault: AccountInfo<'info>,

    #[account(mut,
        constraint = game_ground.winner == winner.key() @ ContractError::IncorrectAuthority)]
    winner: Signer<'info>,

    #[account(mut,
        constraint = global_config.payer_wallet == payer.key() @ ContractError::IncorrectPayerAuthority)]
    payer: Signer<'info>,

    #[account(
        mut,
        seeds = [GAME_GROUND.as_bytes(), round_num.to_le_bytes().as_ref()],
        bump
    )]
    game_ground: Box<Account<'info, GameGround>>,

    #[account(address = system_program::ID)]
    system_program: Program<'info, System>,
}

impl<'info> ClaimReward<'info> {
    /// Handles claiming the reward for a completed game.
    ///
    /// Validates that the caller is the winner and transfers the
    /// total deposit amount from the vault to the winner.
    ///
    /// # Arguments
    /// * `round_num` - The round number to claim reward from
    /// * `global_vault_bump` - The bump seed for the global vault PDA
    pub fn handler(&mut self, round_num: u64, global_vault_bump: u8) -> Result<()> {
        require!(
            round_num < self.global_config.game_round,
            ContractError::RoundNumberError
        );

        let game_ground = &mut self.game_ground;

        require!(
            game_ground.is_claimed == false,
            ContractError::WinnerClaimed
        );

        require!(
            game_ground.is_completed == true,
            ContractError::GameNotCompleted
        );

        let signer_seeds: &[&[&[u8]]] = &[&[GLOBAL.as_bytes(), &[global_vault_bump]]];

        sol_transfer_with_signer(
            self.global_vault.to_account_info(),
            self.winner.to_account_info(),
            &self.system_program,
            signer_seeds,
            game_ground.total_deposit,
        )?;

        game_ground.is_claimed = true;

        Ok(())
    }
}
