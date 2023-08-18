use dotenv::dotenv;
use teloxide::{dispatching::dialogue::InMemStorage, prelude::*};

type MyDialogue = Dialogue<State, InMemStorage<State>>;
type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

#[derive(Clone, Default)]
pub enum State {
    #[default]
    Start,
}

fn main() {
    dotenv().ok(); // Load env variables

    let telebot_api_token = std::env::var("TELOXIDE_TOKEN").expect("TELOXIDE_TOKEN must set.");
}
