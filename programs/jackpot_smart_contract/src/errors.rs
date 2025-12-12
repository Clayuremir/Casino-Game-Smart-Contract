//! Custom error codes for the jackpot smart contract.

use anchor_lang::prelude::*;

pub use ContractError::*;

/// Custom error codes returned by the program.
#[error_code]
pub enum ContractError {
    #[msg("Value is too small")]
    ValueTooSmall,

    #[msg("Value is too large")]
    ValueTooLarge,

    #[msg("Value is invalid")]
    ValueInvalid,

    #[msg("Incorrect authority")]
    IncorrectAuthority,

    #[msg("Incorrect team wallet authority")]
    IncorrectTeamWalletAuthority,

    #[msg("Incorrect payer authority")]
    IncorrectPayerAuthority,

    #[msg("Incorrect config account")]
    IncorrectConfigAccount,

    #[msg("Overflow or underflow occurred")]
    OverflowOrUnderflowOccurred,

    #[msg("Amount is invalid")]
    InvalidAmount,

    #[msg("Incorrect team wallet address")]
    IncorrectTeamWallet,

    #[msg("Cannot deposit after the game is completed")]
    GameAlreadyCompleted,

    #[msg("Winner has already been set")]
    SetWinnerCompleted,

    #[msg("Game is not completed")]
    GameNotCompleted,

    #[msg("Winner has already claimed the reward")]
    WinnerClaimed,

    #[msg("Return amount is too small compared to the minimum received amount")]
    ReturnAmountTooSmall,

    #[msg("Global configuration not initialized")]
    NotInitialized,

    #[msg("Invalid global authority")]
    InvalidGlobalAuthority,

    #[msg("Not enough SOL received to be valid")]
    InsufficientSol,

    #[msg("Arithmetic error occurred")]
    ArithmeticError,

    #[msg("Math overflow occurred")]
    MathOverflow,

    #[msg("End time is invalid")]
    EndTimeError,

    #[msg("Round time is invalid")]
    RoundTimeError,

    #[msg("Minimum deposit amount is invalid")]
    MinDepositAmountError,

    #[msg("Maximum joiner count is invalid")]
    MaxJoinerCountError,

    #[msg("User count exceeds the maximum allowed limit")]
    UserCountOverError,

    #[msg("Randomness is still being fulfilled")]
    StillProcessing,

    #[msg("The deposit amount must be greater than the minimum deposit amount")]
    DepositAmountError,

    #[msg("Round number does not exist")]
    RoundNumberError,
}
