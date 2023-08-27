//! Bot services.
//!
//! The shuttle_runtime for the BotService is defined here, which
//! binds itself to the `SocketAddr` provided by shuttle.
use crate::{
    database::*,
    message::{display_tally, list_runs, list_users},
};
use sqlx::PgPool;
use teloxide::{prelude::*, utils::command::BotCommands};
use tracing::error;

/// Encapsulate the BotService.
pub struct BotService {
    /// Teloxide Bot.
    pub bot: Bot,
    /// Database connection.
    pub postgres: PgPool,
}

/// Required implementation of the `shuttle_runtime::Service` trait for `BotService`.
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

/// impl block for `BotService`.
impl BotService {
    /// Clones `bot` and `db_connection` before passing these over to `Command`, also
    /// defined within teloxide. It parses incoming commands, matches them and
    /// hands them over to the `answer` methold.
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

/// Enumeration of commands accepted by the bot.
#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "The following commands are supported:"
)]
enum Command {
    /// Matched to `/help` -> displays commands and their documentation.
    #[command(description = "Display this text. Usage: /help")]
    Help,
    #[command(description = "Show users registered on telerun within the chat. Usage: /show")]
    /// Matched to `/show` -> displays users within chat.
    Show,
    /// Matched to `/add <distance>` -> creates users in db if not present,
    /// then adds run data to runs table.
    #[command(description = "Add run data to database. Usage: /add <distance>")]
    Add {
        /// Distance run in km
        distance: f32,
    },
    /// Matched to `/edit <run_id> <distance>` -> edits stored run data.
    #[command(
        description = "Edit data for a run. Usage: /edit <run_id> <distance>",
        parse_with = "split"
    )]
    Edit {
        /// Id of run as stored in runs table.
        run_id: i32,
        /// Corrected distance run in km.
        distance: f32,
    },
    /// Matched to `/delete <run_id>` -> removes a certain run from database.
    #[command(description = "Remove a run from database. Usage: /delete <run_id>")]
    Delete {
        /// Id of run to remove from table.
        run_id: i32,
    },
    /// Matched to `/tally` -> sends score board as message through Telegram.
    #[command(description = "Tallies current medals and distances. Usage: /tally")]
    Tally,
    /// Matched to `/list <limit>` -> displays runs registered by the group chat, subject to a limit.
    #[command(
        description = "Lists recent runs. Number of runs to display must be specified. Usage: /list <num_runs_to_show>"
    )]
    List {
        /// Limit to query from db.
        limit: u32,
    },
}

/// Function used for handling various commands matched.
async fn answer(bot: Bot, msg: Message, cmd: Command, db_connection: PgPool) -> ResponseResult<()> {
    match cmd {
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?;
        }
        Command::Show => {
            let users = get_users_in_chat(msg.chat.id, &db_connection).await;
            if let Ok(users) = users {
                let show_message = list_users(users);
                bot.send_message(msg.chat.id, show_message)
                    .await
                    .map_err(|error| error!("Unable to send Show message: {:?}", error))
                    .ok();
            } else {
                error!("Unable to retrieve items required for Show.");
            }
        }
        Command::Add { distance } => {
            let telegram_user = msg.from();
            if let Some(user) = telegram_user {
                let user_name = &user.username;
                if let Some(user_name) = user_name {
                    let add_result = add_run_wrapper(
                        distance,
                        user_name.as_str(),
                        user.id,
                        msg.chat.id,
                        &db_connection,
                    )
                    .await;
                    if add_result.is_ok() {
                        bot.send_message(
                            msg.chat.id,
                            format!("{} ran {}km added to database.", user_name, distance),
                        )
                        .await
                        .map_err(|error| error!("Unable to send Add message: {:?}", error))
                        .ok();
                    } else {
                        error!("Unable to Add run information.");
                    }
                } else {
                    error!("Unable to retrieve username information from Telegram.");
                }
            } else {
                error!("Unable to retrieve user from message.");
            }
        }
        Command::Edit { run_id, distance } => {
            let telegram_user = msg.from();
            if let Some(user) = telegram_user {
                let update_outcome = update_run(run_id, user.id, distance, &db_connection).await;
                if update_outcome.is_ok() {
                    bot.send_message(
                        msg.chat.id,
                        format!(
                            "Run {} successfully updated with distance {}km.",
                            run_id, distance
                        ),
                    )
                    .await
                    .map_err(|error| error!("Unable to send update message: {:?}", error))
                    .ok();
                } else {
                    error!("Unable to update database entry for run_id: {}", run_id);
                }
            } else {
                error!("Unable to retrieve user information from Telegram.");
            }
        }
        Command::Delete { run_id } => {
            let telegram_user = msg.from();
            if let Some(user) = telegram_user {
                let delete_outcome = delete_run(run_id, user.id, &db_connection).await;
                if delete_outcome.is_ok() {
                    bot.send_message(msg.chat.id, format!("Run {} successfully deleted!", run_id))
                        .await
                        .map_err(|error| error!("Unable to send delete message: {:?}", error))
                        .ok();
                } else {
                    error!("Unable to delete entry from database.");
                }
            }
        }
        Command::Tally => {
            let tally = get_tally(msg.chat.id, &db_connection).await;
            if let Ok(tally) = tally {
                let tally_message = display_tally(tally);
                bot.send_message(msg.chat.id, tally_message)
                    .await
                    .map_err(|err| error!("Unable to send Tally message: {:?}", err))
                    .ok();
            } else {
                error!("Unable to retrieve tally from database.");
            }
        }
        Command::List { limit } => {
            let runs = get_runs(msg.chat.id, limit.into(), &db_connection).await;
            if let Ok(runs) = runs {
                let run_message = list_runs(runs);
                bot.send_message(msg.chat.id, run_message)
                    .await
                    .map_err(|err| error!("Unable to send List message: {:?}", err))
                    .ok();
            } else {
                error!("Unable to retrieve runs from database.");
            }
        }
    }
    Ok(())
}
