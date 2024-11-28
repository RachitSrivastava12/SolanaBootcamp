# 🛠️ Favorites Program on Solana

This is a Solana program written in Rust using the [Anchor framework](https://book.anchor-lang.com/). The program allows users to store their favorite data (a number, a color, and a list of hobbies) on-chain. Each user's data is securely stored in their unique account on the Solana blockchain.

## ✨ Features

- Store and update a user's favorite number, color, and hobbies.
- Automatically initialize accounts if they don’t exist.
- Use deterministic account addresses for predictable storage.
- Optimized for Solana's high-performance, low-cost blockchain.

---

## 🚀 How It Works

### Workflow
1. A user calls the `set_favorites` function, passing their data:
   - A favorite number.
   - A favorite color.
   - A list of hobbies (up to 5 hobbies, each 50 characters max).
2. If the user's on-chain account (`favorites`) does not exist, it is created.
3. The user's data is saved or updated in the `favorites` account.
4. Debug logs provide insights into the program's execution.

---

## 🧩 Program Structure

### 1. **Program ID**
Every Solana program is identified by a unique `Program ID`:
```rust
declare_id!("GZJcUuZSRN1nHRueVX4M3AMcwvqMmy4iQiCPwiMnLiVs");
2. Main Function: set_favorites

Handles storing a user’s favorite data:

pub fn set_favorites(
    context: Context<SetFavorites>,
    number: u64,
    color: String,
    hobbies: Vec<String>,
) -> Result<()> {
    // Log program usage
    msg!("Greetings from {}", context.program_id);

    // Retrieve user public key
    let user_public_key = context.accounts.user.key();
    msg!(
        "User {}'s favorite color is {}, and their hobbies are {:?}",
        user_public_key,
        color,
        hobbies
    );

    // Update account data
    context.accounts.favorites.set_inner(Favorites {
        number,
        color,
        hobbies,
    });

    Ok(())
}

3. Data Structs
Favorites

Represents the user’s favorite data:

#[account]
#[derive(InitSpace)]
pub struct Favorites {
    pub number: u64,       // Favorite number
    #[max_len(50)]
    pub color: String,     // Favorite color
    #[max_len(5, 50)]
    pub hobbies: Vec<String>, // List of hobbies
}

SetFavorites Context

Defines the accounts required to execute set_favorites:

#[derive(Accounts)]
pub struct SetFavorites<'info> {
    #[account(mut)]
    pub user: Signer<'info>, // User signing the transaction
    #[account(
        init_if_needed,
        payer = user,
        space = 8 + Favorites::INIT_SPACE,
        seeds = [b"favorites", user.key().as_ref()],
        bump
    )]
    pub favorites: Account<'info, Favorites>, // User's favorites account
    pub system_program: Program<'info, System>, // System program for account creation
}

🛠 Prerequisites

    Install Rust and Solana CLI:
        Install Rust
        Install Solana CLI

    Install Anchor:

cargo install --git https://github.com/coral-xyz/anchor --tag v0.27.0 anchor-cli --locked

Set up Solana environment:

    solana config set --url https://api.devnet.solana.com

🚀 Deployment

    Build the Program:

anchor build

Deploy to Devnet:

    anchor deploy

    Test the Program:
        Write client-side scripts using JavaScript or TypeScript with Anchor’s IDL.

🌟 Example Usage

Here’s how you could interact with the program:
Client Script (TypeScript)

const tx = await program.rpc.setFavorites(
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
console.log("Transaction signature:", tx);

📚 Learning Resources

    Rust Book
    Anchor Documentation
    Solana Developer Docs

🔒 Security

    Ensure your private keys are secure.
    Only use trusted wallets to interact with the program.

