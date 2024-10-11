use teloxide::{prelude::*, utils::command::BotCommand};
use teloxide::requests::ResponseResult;
use dotenv::dotenv;

#[derive(BotCommand)]
#[command(rename = "lowercase", description = "Comandos disponíveis:")]
enum Command {
    #[command(description = "Exibe a saudação e os comandos disponíveis.")]
    Start,
    #[command(description = "Exibe uma mensagem de ajuda.")]
    Help,
    #[command(description = "Exibe informações sobre o bot.")]
    Info,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let bot = Bot::from_env().auto_send();

    teloxide::commands_repl(bot, "bot_name", answer).await;
}

async fn answer(
    cx: UpdateWithCx<AutoSend<Bot>, Message>, 
    command: Command
) -> ResponseResult<()> {
    match command {
        Command::Start => {
            cx.answer("Olá! Eu sou um bot. Use /help para ver os comandos disponíveis.").await?;
        }
        Command::Help => {
            cx.answer("/start - Exibe a saudação\n/help - Exibe esta mensagem de ajuda\n/info - Exibe informações sobre o bot").await?;
        }
        Command::Info => {
            cx.answer("Eu sou um bot criado para ajudar com tarefas diversas no Telegram.").await?;
        }
    }
    Ok(())
}
