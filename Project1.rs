// Import necessary modules from the Anchor framework to work with Solana.
use anchor_lang::prelude::*;

// Declare a unique Program ID that identifies this program on the Solana blockchain.
// This ID is used to ensure the program is recognized and can interact on-chain.
declare_id!("GZJcUuZSRN1nHRueVX4M3AMcwvqMmy4iQiCPwiMnLiVs"); // Program ID

// The #[program] macro converts this module into a Solana program.
// This allows the module to handle on-chain logic and define program functions.
#[program]
pub mod favorites {
    use super::*;

    // Define the 'set_favorites' function, which allows users to store their favorite data.
    // It takes a context containing accounts, and three parameters: number, color, and hobbies.
    pub fn set_favorites(
        //instruction handler
        context: Context<SetFavorites>, // Holds all accounts required for this function to work
        number: u64,                    // User's favorite number (64-bit unsigned integer)
        color: String,                  // User's favorite color (string)
        hobbies: Vec<String>,           // User's hobbies (vector of strings)
    ) -> Result<()> {
        // Returns a Result type, with () indicating success
        // Log a message indicating the program ID being used.
        msg!("Greetings from {}", context.program_id);

        // Retrieve the user's public key from the context's accounts.
        let user_public_key = context.accounts.user.key();

        // Log details about the user's favorites for debugging and tracking purposes.
        // The `{hobbies:?}` syntax formats the hobbies vector for easy readability.
        msg!("User",user_public_key"'s favorite color is ",color", and their hobbies are ",hobbies);

        // Update the 'favorites' account data with the new values.
        // `set_inner` replaces the existing data in the Favorites account with the new values.
        context.accounts.favorites.set_inner(Favorites {
            number,
            color,
            hobbies,
        });

        // Return `Ok(())` to signify that the function executed successfully.
        Ok(())
    }
}

// Define the `Favorites` struct, which represents the user's favorite data.
// This struct will be stored on-chain in an account, and each user can have their own instance.
#[account] // The #[account] attribute specifies that this struct is stored on-chain.
#[derive(InitSpace)] // Automatically calculates the required space for the account.
pub struct Favorites {
    pub number: u64, // User's favorite number, stored as an unsigned 64-bit integer.

    #[max_len(50)] // Sets a maximum length of 50 characters for the color string.
    pub color: String, // User's favorite color.

    #[max_len(5, 50)] // Maximum of 5 hobbies, each up to 50 characters.
    pub hobbies: Vec<String>, // User's hobbies, stored as a vector of strings.
}

// Define the `SetFavorites` struct, which represents the context required by `set_favorites`.
// This struct lists the accounts needed for the function to execute properly.
#[derive(Accounts)] //Tells Anchor to interpret this struct as an "Accounts" context for set_favorites, ensuring it has the accounts necessary for execution.
pub struct SetFavorites<'info> {
    // The <'info> lifetime ensures all account references in SetFavorites are only valid for the lifetime of the transaction.
    #[account(mut)] // Specifies that the 'user' account is mutable, allowing modifications.
    pub user: Signer<'info>, // The user signing the transaction; pays for execution costs.

    #[account(
        init_if_needed,       // Initializes the account if it doesn't already exist.
        payer = user,         // Specifies that the 'user' account will pay for account initialization.
        space = 8 + Favorites::INIT_SPACE, // Allocates memory: 8 bytes for metadata + calculated space.
        seeds = [b"favorites", user.key().as_ref()], // Generates a deterministic address for the account.
        bump                  // The 'bump' to prevent collisions in case of multiple accounts.
    )]
    pub favorites: Account<'info, Favorites>, // Account to hold the user's favorites.

    pub system_program: Program<'info, System>, // System program used to create accounts and handle transactions.
}
