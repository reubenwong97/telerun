use sqlx::PgPool;
use teloxide::{macros::BotCommands, prelude::*};

pub struct BotService {
    pub bot: Bot,
    pub postgres: PgPool,
}

#[shuttle_runtime::async_trait]
impl shuttle_runtime::Service for BotService {
    async fn bind(self, _addr: std::net::SocketAddr) -> Result<(), shuttle_runtime::Error> {
        // Start your service and bind to the socket address
        Ok(())
    }
}

impl BotService {
    async fn start(&self) -> Result<(), shuttle_runtime::CustomError> {
        let bot = self.bot.clone();
        let db_connection = self.postgres.clone();

        Ok(())
    }
}

#[derive(BotCommands, Clone)]
enum Command {}
