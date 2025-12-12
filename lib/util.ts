/**
 * Utility functions for transaction execution and Solana operations.
 */

import { Transaction, Connection, Commitment } from "@solana/web3.js";
import NodeWallet from "@coral-xyz/anchor/dist/cjs/nodewallet";

/**
 * Executes a transaction and waits for confirmation.
 *
 * @param transaction - The transaction to execute
 * @param connection - The Solana connection
 * @param payer - The wallet to sign and pay for the transaction
 * @param commitment - The commitment level for confirmation (default: "confirmed")
 * @returns The transaction signature, or undefined if execution failed
 */
export const execTx = async (
    transaction: Transaction,
    connection: Connection,
    payer: NodeWallet,
    commitment: Commitment = "confirmed"
): Promise<string | undefined> => {
    try {
        // Sign the transaction with payer wallet
        const signedTx = await payer.signTransaction(transaction);

        // Serialize the transaction
        const rawTransaction = signedTx.serialize();

        // Simulate the transaction
        const simulation = await connection.simulateTransaction(signedTx);
        console.log("Transaction simulation:", simulation);

        // Send the transaction
        const txid = await connection.sendRawTransaction(rawTransaction, {
            skipPreflight: true,
            maxRetries: 2,
            preflightCommitment: "processed",
        });

        console.log(
            `Transaction sent: https://solscan.io/tx/${txid}?cluster=custom&customUrl=${connection.rpcEndpoint}`
        );

        // Wait for confirmation
        const confirmed = await connection.confirmTransaction(txid, commitment);

        if (confirmed.value.err) {
            console.error("Transaction error:", confirmed.value.err);
            return undefined;
        }

        console.log("Transaction confirmed:", txid);
        return txid;
    } catch (error) {
        console.error("Error executing transaction:", error);
        return undefined;
    }
};
