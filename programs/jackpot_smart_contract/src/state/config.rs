//! Configuration state for the jackpot program.

use crate::errors::*;
use anchor_lang::{prelude::*, AnchorDeserialize, AnchorSerialize};
use core::fmt::Debug;

/// Global configuration account storing program-wide settings.
#[account]
#[derive(Debug)]
pub struct Config {
    /// Authority that can configure the program.
    pub authority: Pubkey,

    /// Wallet used for paying transaction fees.
    pub payer_wallet: Pubkey,

    /// Team wallet that receives platform fees.
    pub team_wallet: Pubkey,

    /// Current game round number (increments with each new game).
    pub game_round: u64,

    /// Platform fee in basis points (e.g., 100 = 1%).
    pub platform_fee: u64,

    /// Minimum deposit amount required to join a game (in lamports).
    pub min_deposit_amount: u64,

    /// Maximum number of players allowed in a game.
    pub max_joiner_count: u64,

    /// Whether the configuration has been initialized.
    pub initialized: bool,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, Debug)]
pub enum AmountConfig<T: PartialEq + PartialOrd + Debug> {
    Range { min: Option<T>, max: Option<T> },
    Enum(Vec<T>),
}

impl<T: PartialEq + PartialOrd + Debug> AmountConfig<T> {
    pub fn validate(&self, value: &T) -> Result<()> {
        match self {
            Self::Range { min, max } => {
                if let Some(min) = min {
                    if value < min {
                        return Err(ValueTooSmall.into());
                    }
                }
                if let Some(max) = max {
                    if value > max {
                        return Err(ValueTooLarge.into());
                    }
                }

                Ok(())
            }
            Self::Enum(options) => {
                if options.contains(value) {
                    Ok(())
                } else {
                    Err(ValueInvalid.into())
                }
            }
        }
    }
}
