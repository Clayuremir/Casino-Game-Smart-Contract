# 🎰 Casino Jackpot Smart Contract

A decentralized jackpot smart contract built on the Solana blockchain using the Anchor framework. This project implements a fair jackpot system with secure random number generation using ORAO Network's VRF (Verifiable Random Function).

## 📋 Table of Contents

- [Features](#features)
- [Architecture](#architecture)
- [Prerequisites](#prerequisites)
- [Installation](#installation)
- [Configuration](#configuration)
- [Usage](#usage)
- [Project Structure](#project-structure)
- [Testing](#testing)
- [Security](#security)
- [Contributing](#contributing)
- [License](#license)
- [Contact](#contact)

## ✨ Features

- **Decentralized Jackpot System**: Fair and transparent jackpot mechanism on Solana
- **VRF-Based Randomness**: Secure random number generation using ORAO Network's VRF
- **Weighted Winner Selection**: Winners are selected proportionally based on deposit amounts
- **Platform Fee Support**: Configurable platform fees for sustainability
- **TypeScript Client**: Full TypeScript SDK for interacting with the program
- **CLI Tools**: Command-line interface for easy interaction

## 🏗️ Architecture

The smart contract implements a round-based jackpot system:

1. **Configuration**: Global settings are stored in a PDA (Program Derived Address)
2. **Game Creation**: Admins can create new game rounds with configurable parameters
3. **Player Participation**: Users can join active rounds by depositing SOL
4. **Winner Selection**: After a round ends, a winner is selected using VRF randomness
5. **Reward Claiming**: Winners can claim their rewards from the vault

### Key Components

- **Config Account**: Stores global program configuration
- **Game Ground Account**: Stores state for each game round
- **Global Vault**: PDA that holds all deposited SOL
- **VRF Integration**: Uses ORAO Network for verifiable randomness

## 📦 Prerequisites

Before you begin, ensure you have the following installed:

- **Rust** (latest stable version) - [Install Rust](https://www.rust-lang.org/tools/install)
- **Node.js** (v16 or later) - [Install Node.js](https://nodejs.org/)
- **Yarn** package manager - [Install Yarn](https://yarnpkg.com/getting-started/install)
- **Solana CLI** tools - [Install Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools)
- **Anchor Framework** v0.30.1 - [Install Anchor](https://www.anchor-lang.com/docs/installation)

## 🚀 Installation

1. **Clone the repository**:
   ```bash
   git clone https://github.com/Novus-Tech-LLC/Casino-Game-Smart-Contract.git
   cd Casino-Smart-Contract
   ```

2. **Install dependencies**:
   ```bash
   yarn install
   ```

3. **Build the program**:
   ```bash
   anchor build
   ```

## ⚙️ Configuration

The project is configured to use Solana's devnet by default. Configuration can be found in `Anchor.toml`.

### Important Configuration Steps

1. **Update Program ID**: If deploying to a new instance, update the program ID in:
   - `Anchor.toml`
   - `programs/jackpot_smart_contract/src/lib.rs` (declare_id! macro)

2. **Configure Wallet**: Set your wallet path in `Anchor.toml` or use CLI options

3. **RPC Endpoint**: Configure your RPC endpoint in `Anchor.toml` or use CLI options

### Environment Variables (Recommended)

For production use, consider moving sensitive values to environment variables:

- `SOLANA_RPC_URL`: Your RPC endpoint
- `WALLET_KEYPAIR_PATH`: Path to your wallet keypair
- `TEAM_WALLET_ADDRESS`: Team wallet for receiving fees

## 📖 Usage

### CLI Commands

The project includes a CLI tool for interacting with the smart contract:

#### Configure the Program
```bash
yarn script config
```

#### Create a New Game Round
```bash
yarn script create -t 60 -d 100000000 -j 100
```
- `-t, --time`: Round duration in seconds
- `-d, --minDeposit`: Minimum deposit amount in lamports
- `-j, --maxJoiner`: Maximum number of players

#### Join a Game Round
```bash
yarn script join -a 100000000 -g 2
```
- `-a, --amount`: Deposit amount in lamports
- `-g, --roundNum`: Round number to join

#### Set Winner for a Completed Round
```bash
yarn script winner -g 2
```
- `-g, --roundNum`: Round number

#### Claim Reward
```bash
yarn script claim -g 2
```
- `-g, --roundNum`: Round number

### CLI Options

All commands support the following options:

- `-e, --env <string>`: Solana cluster (mainnet-beta, testnet, devnet) - Default: devnet
- `-r, --rpc <string>`: Custom RPC URL
- `-k, --keypair <string>`: Path to wallet keypair JSON file

### Programmatic Usage

```typescript
import { Program } from "@coral-xyz/anchor";
import { Connection, Keypair } from "@solana/web3.js";
import { createGameTx, joinGameTx } from "./lib/scripts";

// Initialize connection and program
const connection = new Connection("https://api.devnet.solana.com");
const program = // ... initialize program

// Create a game
const tx = await createGameTx(
    creatorPubkey,
    feePayerKeypair,
    60,      // round time
    100000000, // min deposit
    100      // max joiners
);

// Join a game
const joinTx = await joinGameTx(
    userPubkey,
    feePayerKeypair,
    teamWalletPubkey,
    1,       // round number
    100000000 // deposit amount
);
```

## 📁 Project Structure

```
Casino-Smart-Contract/
├── programs/                    # Solana program directory
│   └── jackpot_smart_contract/  # Main program code
│       └── src/
│           ├── lib.rs           # Program entry point
│           ├── constants.rs    # Program constants
│           ├── errors.rs       # Custom error codes
│           ├── instructions/   # Instruction handlers
│           │   ├── admin/      # Admin instructions
│           │   └── user/       # User instructions
│           ├── state/          # Account state structures
│           ├── utils.rs        # Utility functions
│           └── misc.rs         # VRF utilities
├── cli/                        # Command-line interface
│   ├── command.ts             # CLI command definitions
│   └── scripts.ts             # CLI script implementations
├── lib/                        # TypeScript library code
│   ├── constant.ts            # TypeScript constants
│   ├── scripts.ts             # Transaction builders
│   └── util.ts                # Utility functions
├── idl/                        # Interface Definition Language files
├── tests/                      # Test files
├── Anchor.toml                 # Anchor configuration
├── Cargo.toml                  # Rust workspace configuration
└── package.json               # Node.js dependencies
```

## 🧪 Testing

Run the test suite:

```bash
anchor test
```

Or use the CLI scripts for manual testing:

```bash
# Configure the program
yarn script config

# Create a game
yarn script create -t 60 -d 100000000 -j 100

# Join the game
yarn script join -a 100000000 -g 2

# Set winner (after round ends)
yarn script winner -g 2

# Claim reward
yarn script claim -g 2
```

## 🔒 Security

This project implements several security measures:

- **VRF Randomness**: Uses ORAO Network's VRF for provably fair randomness
- **Weighted Selection**: Winners are selected proportionally to their deposits
- **Access Control**: Admin-only functions are protected by authority checks
- **Input Validation**: All inputs are validated before processing
- **Overflow Protection**: Uses checked arithmetic to prevent overflow/underflow

### Security Considerations

- Always verify the program ID before interacting with the contract
- Use a secure RPC endpoint in production
- Keep your wallet keypairs secure and never commit them to version control
- Review all transactions before signing

## 🤝 Contributing

Contributions are welcome! Please read our [Contributing Guide](CONTRIBUTING.md) for details on our code of conduct and the process for submitting pull requests.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## 📄 License

This project is licensed under the ISC License - see the [LICENSE](LICENSE) file for details.

## 📞 Contact

- **Telegram**: [@novustch](https://t.me/novustch)
- **Twitter**: [@novustch](https://x.com/novustch)

## 🔗 Contract Address

- **Devnet**: [CKaQ1zwbTdYoVjBfWMUiZGzTbf8wHfc2ExTRTM79kj7w](https://solscan.io/account/CKaQ1zwbTdYoVjBfWMUiZGzTbf8wHfc2ExTRTM79kj7w?cluster=devnet)

## 🙏 Acknowledgments

- [Anchor Framework](https://www.anchor-lang.com/) for the Solana development framework
- [ORAO Network](https://orao.network/) for VRF randomness
- [Solana](https://solana.com/) for the blockchain platform
