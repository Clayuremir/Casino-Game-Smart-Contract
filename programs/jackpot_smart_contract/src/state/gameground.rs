//! Game state structures for individual jackpot rounds.

use anchor_lang::{prelude::*, AnchorDeserialize, AnchorSerialize};

/// Account storing the state of a single jackpot game round.
#[account]
#[derive(Debug)]
pub struct GameGround {
    /// Public key of the game creator.
    pub creator: Pubkey,

    /// Round number for this game.
    pub game_round: u64,

    /// Unix timestamp when the game was created.
    pub create_date: i64,

    /// Unix timestamp when the game started (after first 2 players join).
    pub start_date: i64,

    /// Unix timestamp when the game ends.
    pub end_date: i64,

    /// Duration of the round in seconds.
    pub round_time: i64,

    /// Total amount of SOL deposited in this round (in lamports).
    pub total_deposit: u64,

    /// Random number used to determine the winner.
    pub rand: u64,

    /// Public key of the winner.
    pub winner: Pubkey,

    /// Current number of players who have joined.
    pub user_count: u64,

    /// Minimum deposit amount for this round (in lamports).
    pub min_deposit_amount: u64,

    /// Maximum number of players allowed in this round.
    pub max_joiner_count: u64,

    /// Seed used for VRF randomness generation.
    pub force: [u8; 32],

    /// Whether the game has been completed and winner set.
    pub is_completed: bool,

    /// Whether the winner has claimed their reward.
    pub is_claimed: bool,

    /// List of all deposits made in this round.
    pub deposit_list: Vec<DepositInfo>,
}

impl GameGround {
    pub const INIT_SPACE: usize = 32 + // creator
        8 +  // game_round
        8 +  // create_date
        8 +  // start_date
        8 +  // end_date
        8 +  // round_time
        8 +  // total_deposit
        8 +  // rand
        32 + // winner
        8 +  // user_count
        8 +  // min_deposit_amount
        8 +  // max_joiner_count
        32 + // force [u8; 32]
        1 +  // is_completed (bool)
        1 +  // is_claimed (bool)
        (24 + 40); // Vec length prefix (for deposit_list)

    pub fn space(len: usize) -> usize {
        8 + Self::INIT_SPACE + len * (32 + 8)
    }
}

/// Information about a single deposit made by a user.
#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize, InitSpace)]
pub struct DepositInfo {
    /// Public key of the user who made the deposit.
    pub user: Pubkey,

    /// Amount deposited (in lamports).
    pub amount: u64,
}

impl DepositInfo {
    pub const INIT_SPACE: usize = 32 + // user Pubkey
        8; // amount
}

/// Trait for game ground account operations.
pub trait GameGroundAccount<'info> {
    /// Adds or updates a deposit for a user.
    ///
    /// If the user already has a deposit, the amount is added to their existing deposit.
    /// Otherwise, a new deposit entry is created.
    fn append(&mut self, entrant: Pubkey, amount: u64);

    /// Sets the winner based on the random number using weighted selection.
    ///
    /// The winner is selected proportionally based on their deposit amount.
    fn set_winner(&mut self, random_num: u64) -> Result<()>;
}

impl<'info> GameGroundAccount<'info> for Account<'info, GameGround> {
    fn append(&mut self, user: Pubkey, amount: u64) {
        if let Some(deposit) = self.deposit_list.iter_mut().find(|d| d.user == user) {
            // If the user already exists, add to their amount
            deposit.amount += amount;
        } else {
            // If user doesn't exist, add a new entry
            self.deposit_list.push(DepositInfo { user, amount });
            self.user_count += 1;
        }
        self.total_deposit += amount;
    }

    fn set_winner(&mut self, random_num: u64) -> Result<()> {
        self.rand = random_num;
        let mut remaining = self.rand % self.total_deposit;

        // Select winner based on weighted random selection
        for deposit in &self.deposit_list {
            if remaining >= deposit.amount {
                remaining -= deposit.amount;
            } else {
                self.winner = deposit.user;
                msg!("Winner selected: {:?}", self.winner);
                break;
            }
        }

        Ok(())
    }
}
