use crate::{database::*, message::list_runs};
use sqlx::PgPool;
use teloxide::{prelude::*, utils::command::BotCommands};

pub struct BotService {
    pub bot: Bot,
    pub postgres: PgPool,
}

#[shuttle_runtime::async_trait]
impl shuttle_runtime::Service for BotService {
    async fn bind(self, _addr: std::net::SocketAddr) -> Result<(), shuttle_runtime::Error> {
        // Start your service and bind to the socket address
        self.start()
            .await
            .expect("An error occured while using the bot!");

        Ok(())
    }
}

impl BotService {
    async fn start(&self) -> Result<(), shuttle_runtime::CustomError> {
        let bot = self.bot.clone();
        let db_connection = self.postgres.clone();

        Command::repl(bot, move |bot, msg, cmd| {
            answer(bot, msg, cmd, db_connection.clone())
        })
        .await;

        Ok(())
    }
}

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "The following commands are supported:"
)]
enum Command {
    #[command(description = "display this text")]
    Help,
    #[command(
        description = "Add run data to database. Format is /add %distance (km)% %username%",
        parse_with = "split"
    )]
    Add { distance: f32, user_name: String },
    #[command(description = "Edit data for a run.", parse_with = "split")]
    Edit { run_id: u32, distance: f32 },
    #[command(description = "Remove a run from database.")]
    Delete { run_id: u32 },
    #[command(description = "Tallies current medals and distances.")]
    Tally,
    #[command(description = "Lists recent runs. Number of runs to display must be specified.")]
    List { limit: u32 },
}

async fn answer(bot: Bot, msg: Message, cmd: Command, db_connection: PgPool) -> ResponseResult<()> {
    match cmd {
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?;
        }
        Command::Add {
            distance,
            user_name,
        } => {
            bot.send_message(msg.chat.id, "Add".to_string()).await?;
        }
        Command::Edit { run_id, distance } => {
            bot.send_message(msg.chat.id, "Edit".to_string()).await?;
        }
        Command::Delete { run_id } => {
            bot.send_message(msg.chat.id, "Delete".to_string()).await?;
        }
        Command::Tally => {
            let tally = get_tally(msg.chat.id, &db_connection)
                .await
                .expect("Failed to retrieve tallies.");
            if let Some(tally) = tally {
                // do something
            }
            bot.send_message(msg.chat.id, "No scores are available!")
                .await?;
        }
        Command::List { limit } => {
            let runs = get_runs(msg.chat.id, limit.into(), &db_connection)
                .await
                .expect("Unable to retrieve runs.");
            let run_message = list_runs(runs);
            bot.send_message(msg.chat.id, run_message).await?;
        }
    }
    Ok(())
}
