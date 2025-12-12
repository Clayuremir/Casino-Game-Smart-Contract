//! Utilities for interacting with ORAO VRF randomness accounts.

use std::mem::size_of;

use anchor_lang::{
    solana_program::{account_info::AccountInfo, program_error::ProgramError},
    AccountDeserialize,
};

use orao_solana_vrf::state::RandomnessAccountData;

/// Deserializes randomness account data from an account info.
///
/// # Arguments
/// * `account_info` - The account info containing randomness data
///
/// # Returns
/// The deserialized randomness account data, or an error if deserialization fails
pub fn get_account_data(account_info: &AccountInfo) -> Result<RandomnessAccountData, ProgramError> {
    if account_info.data_is_empty() {
        return Err(ProgramError::UninitializedAccount);
    }

    RandomnessAccountData::try_deserialize(&mut &account_info.data.borrow()[..])
}

/// Extracts the current random value from a randomness account.
///
/// # Arguments
/// * `randomness` - The randomness account data
///
/// # Returns
/// The random u64 value, or 0 if randomness is not yet fulfilled
pub fn current_state(randomness: &RandomnessAccountData) -> u64 {
    if let Some(randomness_bytes) = randomness.fulfilled_randomness() {
        let value = randomness_bytes[0..size_of::<u64>()]
            .try_into()
            .expect("Failed to extract u64 from randomness");
        u64::from_le_bytes(value)
    } else {
        0
    }
}
