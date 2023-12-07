// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will try to destroy the first account output there is in the wallet.
//!
//! Make sure that `STRONGHOLD_SNAPSHOT_PATH` and `WALLET_DB_PATH` already exist by
//! running the `./how_tos/accounts_and_addresses/create_wallet.rs` example!
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --all-features --example destroy_account_output
//! ```

use iota_sdk::{client::secret::SecretManager, wallet::Result, Wallet};

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    let wallet = Wallet::builder()
        .with_secret_type::<SecretManager>()
        .with_storage_path(&std::env::var("WALLET_DB_PATH").unwrap())
        .finish()
        .await?;

    // May want to ensure the wallet is synced before sending a transaction.
    let balance = wallet.sync(None).await?;

    // Get the first account
    if let Some(account_id) = balance.accounts().first() {
        let accounts_before = balance.accounts();
        println!("Accounts BEFORE destroying:\n{accounts_before:#?}",);

        // Set the stronghold password
        wallet
            .set_stronghold_password(std::env::var("STRONGHOLD_PASSWORD").unwrap())
            .await?;

        println!("Sending account burn transaction...");

        let transaction = wallet.burn(*account_id, None).await?;
        println!("Transaction sent: {}", transaction.transaction_id);

        let block_id = wallet
            .reissue_transaction_until_included(&transaction.transaction_id, None, None)
            .await?;

        println!(
            "Block included: {}/block/{}",
            std::env::var("EXPLORER_URL").unwrap(),
            block_id
        );

        println!("Burned Account '{}'", account_id);

        let balance = wallet.sync(None).await?;

        let accounts_after = balance.accounts();
        println!("Accounts AFTER destroying:\n{accounts_after:#?}",);
    } else {
        println!("No Account available");
    }

    Ok(())
}
