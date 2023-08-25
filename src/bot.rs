use crate::{
    database::*,
    message::{display_tally, list_runs, list_users},
};
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
    #[command(description = "Show users registered on telerun within the chat.")]
    Show,
    #[command(
        description = "Add run data to database. Format is /add %distance (km)% %username%",
        parse_with = "split"
    )]
    Add { distance: f32, user_name: String },
    #[command(description = "Edit data for a run.", parse_with = "split")]
    Edit { run_id: i32, distance: f32 },
    #[command(description = "Remove a run from database.")]
    Delete { run_id: i32 },
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
        Command::Add {
            distance,
            user_name,
        } => {
            let add_result =
                add_run_wrapper(distance, user_name.as_str(), msg.chat.id, &db_connection).await;
            if let Ok(_) = add_result {
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
        }
        Command::Edit { run_id, distance } => {
            let update_outcome = update_run(run_id, distance, &db_connection).await;
            if let Ok(_) = update_outcome {
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
        }
        Command::Delete { run_id } => {
            let delete_outcome = delete_run(run_id, &db_connection).await;
            if let Ok(_) = delete_outcome {
                bot.send_message(msg.chat.id, format!("Run {} successfully deleted!", run_id))
                    .await
                    .map_err(|error| error!("Unable to send delete message: {:?}", error))
                    .ok();
            } else {
                error!("Unable to delete entry from database.");
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
