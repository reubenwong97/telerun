#![warn(missing_docs)]

//! Telegram bot for a running contest between friends.
//!
//! Implements a telegram bot hosted on shuttle.rs (subject to change).
//! Shuttle provisions infrastructure from our infrastructure as code
//! that is used in this codebase.

mod bot;
mod database;
mod message;
mod models;

use bot::BotService;
use shuttle_secrets::SecretStore;
use sqlx::PgPool;
use teloxide::prelude::*;

/// Entry point to the telegram bot service.
///
/// We pass in the resources that we wish to provision as arguments to `shuttle_main()`.
/// As a requirement of shuttle.rs for provisioning a database, we run sqlx migrations
/// as the first step as well.
///
/// Next, we load in our telegram bot's key and as it is a requirement for teloxide.
///
/// Finally, we start our service.
#[shuttle_runtime::main]
async fn shuttle_main(
    #[shuttle_secrets::Secrets] secrets: SecretStore,
    #[shuttle_shared_db::Postgres] postgres: PgPool,
) -> Result<BotService, shuttle_runtime::Error> {
    sqlx::migrate!()
        .run(&postgres)
        .await
        .expect("ERROR: Could not carry out migrations!");

    let teloxide_key = secrets
        .get("TELOXIDE_TOKEN")
        .expect("TELOXIDE_TOKEN needs to be set.");

    Ok(BotService {
        bot: Bot::new(teloxide_key),
        postgres,
    })
}
