/**
 * CLI scripts for interacting with the jackpot smart contract.
 */

import * as anchor from "@coral-xyz/anchor";
import { BN, Program, web3 } from "@coral-xyz/anchor";
import fs from "fs";
import { Keypair, Connection, PublicKey, Cluster } from "@solana/web3.js";
import NodeWallet from "@coral-xyz/anchor/dist/cjs/nodewallet";
import { JackpotSmartContract } from "../target/types/jackpot_smart_contract";
import {
    createConfigTx,
    createGameTx,
    setWinnerTx,
    claimRewardTx,
    joinGameTx,
} from "../lib/scripts";
import { execTx } from "../lib/util";
import {
    SEED_CONFIG,
    DEFAULT_MAX_JOINER_COUNT,
    DEFAULT_PLATFORM_FEE,
    DEFAULT_MIN_DEPOSIT_AMOUNT,
    GAME_GROUND,
} from "../lib/constant";

// Global state for CLI operations
let solConnection: Connection | null = null;
let program: Program<JackpotSmartContract> | null = null;
let payer: NodeWallet | null = null;
let provider: anchor.Provider | null = null;
let feePayer: NodeWallet | null = null;
let feePayerWalletKeypair: Keypair | null = null;
let teamWallet: PublicKey | null = null;
let programId: string | null = null;

/**
 * Sets up the cluster configuration, provider, and program instance.
 * If RPC URL is provided, it will be used; otherwise, the cluster parameter will be used.
 *
 * @param cluster - Solana cluster (e.g., "mainnet-beta", "devnet")
 * @param keypair - Path to the wallet keypair JSON file
 * @param rpc - Optional custom RPC URL
 */
export const setClusterConfig = async (
    cluster: Cluster,
    keypair: string,
    rpc?: string
): Promise<void> => {
    if (!rpc) {
        solConnection = new web3.Connection(web3.clusterApiUrl(cluster));
    } else {
        solConnection = new web3.Connection(rpc);
    }

    const walletKeypair = Keypair.fromSecretKey(
        Uint8Array.from(JSON.parse(fs.readFileSync(keypair, "utf-8"))),
        { skipValidation: true }
    );
    payer = new NodeWallet(walletKeypair);

    feePayerWalletKeypair = Keypair.fromSecretKey(
        Uint8Array.from(JSON.parse(fs.readFileSync("../key/uu.json", "utf-8"))),
        { skipValidation: true }
    );
    feePayer = new NodeWallet(feePayerWalletKeypair);

    // TODO: Move team wallet to environment variable or config file
    teamWallet = new PublicKey("EgBcC7KVQTh1QeU3qxCFsnwZKYMMQkv6TzgEDkKvSNLv");

    console.log("Wallet Address: ", payer.publicKey.toBase58());

    anchor.setProvider(
        new anchor.AnchorProvider(solConnection, payer, {
            skipPreflight: true,
            commitment: "confirmed",
        })
    );

    provider = anchor.getProvider();

    // Generate the program client from IDL.
    program = anchor.workspace.JackpotSmartContract as Program<JackpotSmartContract>;
    programId = program.programId.toBase58();
    console.log("ProgramId: ", program.programId.toBase58());
};

/**
 * Configures the program with initial settings.
 * Note: This function contains hardcoded addresses that should be moved to configuration.
 */
export const configProject = async (): Promise<void> => {
    if (!program || !payer || !solConnection || !teamWallet) {
        throw new Error("Cluster configuration not set. Call setClusterConfig first.");
    }

    console.log("Configuring project...");

    // TODO: Move these to environment variables or config file
    const authority = new PublicKey("H7YMxhKgLw2NDM9WQnpcUefPvCaLJCCYYaq1ETLHXJuH");
    const payerWallet = new PublicKey("H7YMxhKgLw2NDM9WQnpcUefPvCaLJCCYYaq1ETLHXJuH");

    const [configPda] = PublicKey.findProgramAddressSync(
        [Buffer.from(SEED_CONFIG)],
        program.programId
    );
    console.log("Config PDA:", configPda.toBase58());

    const configAccount = await program.account.config.fetch(configPda);
    console.log("Current config:", configAccount);

    const newConfig = {
        authority,
        payerWallet,
        teamWallet,
        gameRound: configAccount.gameRound,
        platformFee: new BN(DEFAULT_PLATFORM_FEE), // 1% in basis points
        minDepositAmount: new BN(DEFAULT_MIN_DEPOSIT_AMOUNT), // 0.1 SOL
        maxJoinerCount: new BN(DEFAULT_MAX_JOINER_COUNT), // 100 players
        initialized: false,
    };

    const tx = await createConfigTx(payer.publicKey, newConfig, solConnection, program);
    await execTx(tx, solConnection, payer);
};

/**
 * Creates a new game round.
 *
 * @param roundTime - Duration of the round in seconds
 * @param minDepositAmount - Minimum deposit amount in lamports
 * @param maxJoinerCount - Maximum number of players allowed
 */
export const createGame = async (
    roundTime: number,
    minDepositAmount: number,
    maxJoinerCount: number
): Promise<void> => {
    if (!program || !payer || !solConnection || !feePayerWalletKeypair) {
        throw new Error("Cluster configuration not set. Call setClusterConfig first.");
    }

    const [configPda] = PublicKey.findProgramAddressSync(
        [Buffer.from(SEED_CONFIG)],
        program.programId
    );
    const configAccount = await program.account.config.fetch(configPda);
    console.log("Current game round:", configAccount.gameRound.toNumber());

    const tx = await createGameTx(
        payer.publicKey,
        feePayerWalletKeypair,
        roundTime,
        minDepositAmount,
        maxJoinerCount,
        solConnection,
        program
    );

    await execTx(tx, solConnection, payer);
};

/**
 * Sets the winner for a completed game round.
 *
 * @param roundNum - The round number to set winner for
 */
export const setWinner = async (roundNum: number): Promise<void> => {
    if (!program || !payer || !solConnection) {
        throw new Error("Cluster configuration not set. Call setClusterConfig first.");
    }

    const tx = await setWinnerTx(payer.publicKey, roundNum, solConnection, program);
    await execTx(tx, solConnection, payer);

    const [gameGroundPda] = PublicKey.findProgramAddressSync(
        [Buffer.from(GAME_GROUND), new BN(roundNum).toArrayLike(Buffer, "le", 8)],
        program.programId
    );
    console.log("Game Ground PDA:", gameGroundPda.toBase58());

    const gameGroundAccount = await program.account.gameGround.fetch(gameGroundPda);
    console.log("Winner:", gameGroundAccount.winner.toBase58());
};

/**
 * Claims the reward for a completed game round.
 *
 * @param roundNum - The round number to claim reward from
 */
export const claimReward = async (roundNum: number): Promise<void> => {
    if (!program || !payer || !solConnection || !feePayerWalletKeypair) {
        throw new Error("Cluster configuration not set. Call setClusterConfig first.");
    }

    const tx = await claimRewardTx(
        payer.publicKey,
        feePayerWalletKeypair,
        roundNum,
        solConnection,
        program
    );

    await execTx(tx, solConnection, payer);
};

/**
 * Joins an active game round by depositing SOL.
 *
 * @param roundNum - The round number to join
 * @param amount - The amount of SOL to deposit in lamports
 */
export const joinGame = async (roundNum: number, amount: number): Promise<void> => {
    if (!program || !payer || !solConnection || !feePayerWalletKeypair || !teamWallet) {
        throw new Error("Cluster configuration not set. Call setClusterConfig first.");
    }

    const tx = await joinGameTx(
        payer.publicKey,
        feePayerWalletKeypair,
        teamWallet,
        roundNum,
        amount,
        solConnection,
        program
    );

    await execTx(tx, solConnection, payer);
}; 