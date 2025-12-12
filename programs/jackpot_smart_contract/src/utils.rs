//! Utility functions for token and SOL transfers, conversions, and account management.

use crate::*;
use anchor_spl::token::{self, Token};
use solana_program::program::{invoke, invoke_signed};
use std::cmp::Ordering;
use std::ops::{Div, Mul};

/// Converts a token amount from its raw value to a float representation.
///
/// # Arguments
/// * `value` - The raw token amount
/// * `decimals` - Number of decimal places
///
/// # Returns
/// The float representation of the token amount
pub fn convert_to_float(value: u64, decimals: u8) -> f64 {
    (value as f64).div(f64::powf(10.0, decimals as f64))
}

/// Converts a float value to its raw token amount representation.
///
/// # Arguments
/// * `value` - The float value
/// * `decimals` - Number of decimal places
///
/// # Returns
/// The raw token amount
pub fn convert_from_float(value: f64, decimals: u8) -> u64 {
    value.mul(f64::powf(10.0, decimals as f64)) as u64
}

/// Transfers SOL from a user account to a destination account.
///
/// # Arguments
/// * `signer` - The signer account that owns the SOL
/// * `destination` - The destination account to receive SOL
/// * `system_program` - The system program
/// * `amount` - Amount of SOL to transfer (in lamports)
pub fn sol_transfer_from_user<'info>(
    signer: &Signer<'info>,
    destination: AccountInfo<'info>,
    system_program: &Program<'info, System>,
    amount: u64,
) -> Result<()> {
    let ix = solana_program::system_instruction::transfer(signer.key, destination.key, amount);
    invoke(
        &ix,
        &[
            signer.to_account_info(),
            destination,
            system_program.to_account_info(),
        ],
    )?;
    Ok(())
}

/// Transfers tokens from a user's token account to another token account.
///
/// # Arguments
/// * `from` - Source token account
/// * `authority` - Authority that owns the source account
/// * `to` - Destination token account
/// * `token_program` - The token program
/// * `amount` - Amount of tokens to transfer
pub fn token_transfer_user<'info>(
    from: AccountInfo<'info>,
    authority: &Signer<'info>,
    to: AccountInfo<'info>,
    token_program: &Program<'info, Token>,
    amount: u64,
) -> Result<()> {
    let cpi_ctx: CpiContext<_> = CpiContext::new(
        token_program.to_account_info(),
        token::Transfer {
            from,
            authority: authority.to_account_info(),
            to,
        },
    );
    token::transfer(cpi_ctx, amount)?;

    Ok(())
}

/// Transfers tokens from a PDA token account to another token account.
///
/// # Arguments
/// * `from` - Source PDA token account
/// * `authority` - PDA authority account
/// * `to` - Destination token account
/// * `token_program` - The token program
/// * `signer_seeds` - Seeds for PDA signing
/// * `amount` - Amount of tokens to transfer
pub fn token_transfer_with_signer<'info>(
    from: AccountInfo<'info>,
    authority: AccountInfo<'info>,
    to: AccountInfo<'info>,
    token_program: &Program<'info, Token>,
    signer_seeds: &[&[&[u8]]],
    amount: u64,
) -> Result<()> {
    let cpi_ctx: CpiContext<_> = CpiContext::new_with_signer(
        token_program.to_account_info(),
        token::Transfer {
            from,
            to,
            authority,
        },
        signer_seeds,
    );
    token::transfer(cpi_ctx, amount)?;

    Ok(())
}

/// Transfers SOL from a PDA account to a destination account.
///
/// # Arguments
/// * `source` - Source PDA account
/// * `destination` - Destination account
/// * `system_program` - The system program
/// * `signers_seeds` - Seeds for PDA signing
/// * `amount` - Amount of SOL to transfer (in lamports)
pub fn sol_transfer_with_signer<'info>(
    source: AccountInfo<'info>,
    destination: AccountInfo<'info>,
    system_program: &Program<'info, System>,
    signers_seeds: &[&[&[u8]]],
    amount: u64,
) -> Result<()> {
    let ix = solana_program::system_instruction::transfer(source.key, destination.key, amount);
    invoke_signed(
        &ix,
        &[source, destination, system_program.to_account_info()],
        signers_seeds,
    )?;
    Ok(())
}

/// Calculates a value multiplied by basis points (BPS).
///
/// # Arguments
/// * `bps` - Basis points (e.g., 100 = 1%)
/// * `value` - The value to multiply
/// * `divisor` - The divisor (typically 10,000 for BPS)
///
/// # Returns
/// The calculated value, or None if overflow occurs
pub fn bps_mul(bps: u64, value: u64, divisor: u64) -> Option<u64> {
    bps_mul_raw(bps, value, divisor)?.try_into().ok()
}

/// Calculates a value multiplied by basis points using u128 for intermediate calculations.
///
/// # Arguments
/// * `bps` - Basis points (e.g., 100 = 1%)
/// * `value` - The value to multiply
/// * `divisor` - The divisor (typically 10,000 for BPS)
///
/// # Returns
/// The calculated value as u128, or None if overflow occurs
pub fn bps_mul_raw(bps: u64, value: u64, divisor: u64) -> Option<u128> {
    (value as u128)
        .checked_mul(bps as u128)?
        .checked_div(divisor as u128)
}

/// Resizes an account to a new size, adjusting rent accordingly.
///
/// # Arguments
/// * `account_info` - The account to resize
/// * `new_space` - The new size in bytes
/// * `payer` - The account that will pay for additional rent
/// * `system_program` - The system program
pub fn resize_account<'info>(
    account_info: AccountInfo<'info>,
    new_space: usize,
    payer: AccountInfo<'info>,
    system_program: AccountInfo<'info>,
) -> Result<()> {
    let rent = Rent::get()?;
    let new_minimum_balance = rent.minimum_balance(new_space);
    let current_balance = account_info.lamports();

    match new_minimum_balance.cmp(&current_balance) {
        Ordering::Greater => {
            let lamports_diff = new_minimum_balance.saturating_sub(current_balance);
            invoke(
                &solana_program::system_instruction::transfer(
                    &payer.key(),
                    &account_info.key(),
                    lamports_diff,
                ),
                &[payer.clone(), account_info.clone(), system_program.clone()],
            )?;
        }
        Ordering::Less => {
            let lamports_diff = current_balance.saturating_sub(new_minimum_balance);
            **account_info.try_borrow_mut_lamports()? = new_minimum_balance;
            **payer.try_borrow_mut_lamports()? = payer
                .lamports()
                .checked_add(lamports_diff)
                .ok_or(ContractError::ArithmeticError)?;
        }
        Ordering::Equal => {}
    }
    account_info.realloc(new_space, false)?;
    Ok(())
}
