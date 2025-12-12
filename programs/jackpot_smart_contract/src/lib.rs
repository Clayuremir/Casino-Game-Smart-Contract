//! # Jackpot Smart Contract
//!
//! A decentralized jackpot smart contract built on the Solana blockchain using the Anchor framework.
//! This program implements a fair jackpot system with secure random number generation using
//! ORAO Network's VRF (Verifiable Random Function).

use anchor_lang::prelude::*;

pub mod constants;
pub mod errors;
pub mod instructions;
pub mod misc;
pub mod state;
pub mod utils;

use instructions::{claim_reward::*, configure::*, create_game::*, join_game::*, set_winner::*};
use state::config::*;

declare_id!("CKaQ1zwbTdYoVjBfWMUiZGzTbf8wHfc2ExTRTM79kj7w");

/// Main program module containing all instruction handlers.
#[program]
pub mod jackpot_smart_contract {
    use super::*;

    /// Configures the global program settings.
    ///
    /// # Arguments
    /// * `ctx` - The context containing accounts for configuration
    /// * `new_config` - The new configuration parameters
    ///
    /// # Errors
    /// Returns an error if the authority is incorrect or configuration fails.
    pub fn configure(ctx: Context<Configure>, new_config: Config) -> Result<()> {
        msg!("Configuring program with: {:#?}", new_config);
        ctx.accounts.handler(new_config, ctx.bumps.config)
    }

    /// Creates a new jackpot game round.
    ///
    /// # Arguments
    /// * `ctx` - The context containing accounts for game creation
    /// * `force` - A 32-byte seed for randomness generation
    /// * `round_time` - Duration of the round in seconds
    /// * `min_deposit_amount` - Minimum deposit amount required to join
    /// * `max_joiner_count` - Maximum number of players allowed
    ///
    /// # Errors
    /// Returns an error if validation fails or game creation fails.
    pub fn create_game(
        ctx: Context<CreateGame>,
        force: [u8; 32],
        round_time: i64,
        min_deposit_amount: u64,
        max_joiner_count: u64,
    ) -> Result<()> {
        ctx.accounts
            .handler(force, round_time, min_deposit_amount, max_joiner_count)
    }

    /// Sets the winner for a completed game round using VRF randomness.
    ///
    /// # Arguments
    /// * `ctx` - The context containing accounts for setting winner
    /// * `round_num` - The round number to set winner for
    ///
    /// # Errors
    /// Returns an error if the game is not completed or randomness is not ready.
    pub fn set_winner(ctx: Context<SetWinner>, round_num: u64) -> Result<()> {
        ctx.accounts.handler(round_num)
    }

    /// Allows a user to join an active game round by depositing SOL.
    ///
    /// # Arguments
    /// * `ctx` - The context containing accounts for joining
    /// * `round_num` - The round number to join
    /// * `amount` - The amount of SOL to deposit
    ///
    /// # Errors
    /// Returns an error if validation fails or deposit amount is insufficient.
    pub fn join_game(ctx: Context<JoinGame>, round_num: u64, amount: u64) -> Result<()> {
        ctx.accounts.handler(round_num, amount)
    }

    /// Allows the winner to claim their reward from a completed game.
    ///
    /// # Arguments
    /// * `ctx` - The context containing accounts for claiming
    /// * `round_num` - The round number to claim reward from
    ///
    /// # Errors
    /// Returns an error if the caller is not the winner or reward already claimed.
    pub fn claim_reward(ctx: Context<ClaimReward>, round_num: u64) -> Result<()> {
        ctx.accounts.handler(round_num, ctx.bumps.global_vault)
    }
}
