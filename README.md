# 🛠️ Project 1 :  Favorites Program on Solana

This is a Solana program written in Rust using the Anchor framework. It enables users to store their favorite number, color, and hobbies on the blockchain.

## ✨ Features

- Store and update favorite data securely on-chain.
- Automatically initializes accounts if they don’t exist.
- Uses deterministic account addresses tied to user public keys.

---

## 📂 Structure

- **Program ID**: Unique identifier for this program on Solana.
- **Favorites Account**: Stores user data (number, color, hobbies).
- **set_favorites Function**: Handles user interactions for storing/updating data.

---

## 🛠 Prerequisites

1. Install Rust: [Rust Installation](https://www.rust-lang.org/tools/install)  
2. Install Solana CLI: [Solana CLI Guide](https://docs.solana.com/cli/install-solana-cli-tools)  
3. Install Anchor:
   ```bash
   cargo install --git https://github.com/coral-xyz/anchor --tag v0.27.0 anchor-cli --locked

## Configure Solana Environment:

- solana config set --url https://api.devnet.solana.com

## 🚀 Deployment

- **Build**: anchor build
- **Deploy**: anchor deploy

## 🌟 Usage Example

await program.rpc.setFavorites(
  42,                          // Favorite number
  "blue",                      // Favorite color
  ["reading", "coding"],       // Hobbies
  {
    accounts: {
      user: provider.wallet.publicKey,
      favorites: favoritesAccountPublicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
    },
  }
);
