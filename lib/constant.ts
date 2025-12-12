/**
 * Constants used throughout the TypeScript client.
 */

import { Cluster } from "@solana/web3.js";

/** Seed for the configuration PDA. */
export const SEED_CONFIG = "config";

/** Seed for the game ground PDA. */
export const GAME_GROUND = "BONDING_CURVE";

/** Default cluster for development. */
export const DEFAULT_CLUSTER: Cluster = "devnet";

/** Default platform fee in basis points (100 = 1%). */
export const DEFAULT_PLATFORM_FEE = 100;

/** Default minimum deposit amount in lamports (0.1 SOL). */
export const DEFAULT_MIN_DEPOSIT_AMOUNT = 100_000_000;

/** Default maximum number of players allowed in a game. */
export const DEFAULT_MAX_JOINER_COUNT = 100;