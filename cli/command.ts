/**
 * CLI command definitions for the jackpot smart contract.
 */

import { program } from "commander";
import {
    configProject,
    setClusterConfig,
    createGame,
    setWinner,
    claimReward,
    joinGame,
} from "./scripts";

program.version("1.0.0").description("CLI tool for interacting with the Jackpot Smart Contract");

programCommand("config").description("Configure the program with initial settings").action(async (directory, cmd) => {
    const { env, keypair, rpc } = cmd.opts();

    console.log("Solana Cluster:", env);
    console.log("Keypair Path:", keypair);
    console.log("RPC URL:", rpc);

    await setClusterConfig(env, keypair, rpc);

    await configProject();
});

programCommand("create")
    .description("Create a new game round")
    .requiredOption("-t, --time <number>", "Round duration in seconds")
    .requiredOption("-d, --minDeposit <number>", "Minimum deposit amount in lamports")
    .requiredOption("-j, --maxJoiner <number>", "Maximum number of players")
    .action(async (directory, cmd) => {
        const { env, keypair, rpc, time, minDeposit, maxJoiner } = cmd.opts();

        console.log("Solana Cluster:", env);
        console.log("Keypair Path:", keypair);
        console.log("RPC URL:", rpc);

        await setClusterConfig(env, keypair, rpc);

        await createGame(time, minDeposit, maxJoiner);
    });

programCommand("winner")
    .description("Set the winner for a completed game round")
    .requiredOption("-g, --roundNum <number>", "Round number")
    .action(async (directory, cmd) => {
        const { env, keypair, rpc, roundNum } = cmd.opts();

        console.log("Solana Cluster:", env);
        console.log("Keypair Path:", keypair);
        console.log("RPC URL:", rpc);

        await setClusterConfig(env, keypair, rpc);

        await setWinner(roundNum);
    });

programCommand("claim")
    .description("Claim the reward for a completed game round")
    .requiredOption("-g, --roundNum <number>", "Round number")
    .action(async (directory, cmd) => {
        const { env, keypair, rpc, roundNum } = cmd.opts();

        console.log("Solana Cluster:", env);
        console.log("Keypair Path:", keypair);
        console.log("RPC URL:", rpc);

        await setClusterConfig(env, keypair, rpc);

        await claimReward(roundNum);
    });

programCommand("join")
    .description("Join an active game round")
    .requiredOption("-a, --amount <number>", "Deposit amount in lamports")
    .requiredOption("-g, --roundNum <number>", "Round number")
    .action(async (directory, cmd) => {
        const { env, keypair, rpc, amount, roundNum } = cmd.opts();

        console.log("Solana Cluster:", env);
        console.log("Keypair Path:", keypair);
        console.log("RPC URL:", rpc);

        await setClusterConfig(env, keypair, rpc);

        await joinGame(roundNum, amount);
    });


/**
 * Creates a command with common options for cluster configuration.
 *
 * @param name - The command name
 * @returns The configured command
 */
function programCommand(name: string) {
    return program
        .command(name)
        .option(
            "-e, --env <string>",
            "Solana cluster environment (mainnet-beta, testnet, devnet)",
            "devnet"
        )
        .option(
            "-r, --rpc <string>",
            "Custom RPC URL (overrides env if provided)",
            // TODO: Move default RPC to environment variable
            "https://devnet.helius-rpc.com/?api-key=7387c4ee-fe6a-43a6-96ea-05e6534aa500"
        )
        .option(
            "-k, --keypair <string>",
            "Path to Solana wallet keypair JSON file",
            "../key/uu.json"
        );
}

program.parse(process.argv);

/*
Example usage:

  yarn script config
  yarn script create -t 60 -d 100000000 -j 100
  yarn script join -a 100000000 -g 2
  yarn script winner -g 2
  yarn script claim -g 2
*/