mod bot;
mod database;
mod message;
mod models;

extern crate pretty_env_logger;
#[macro_use]
extern crate log;

use bot::BotService;
use shuttle_secrets::SecretStore;
use sqlx::PgPool;
use teloxide::prelude::*;

#[shuttle_runtime::main]
async fn shuttle_main(
    #[shuttle_secrets::Secrets] secrets: SecretStore,
    #[shuttle_shared_db::Postgres] postgres: PgPool,
) -> Result<BotService, shuttle_runtime::Error> {
    pretty_env_logger::init();
    info!("Logging initialised...");

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
