use chrono::{Datelike, Timelike, Utc};
use carapax::{handler, HandlerResult, ExecuteError};
use carapax::types::{Command, ParseMode};
use carapax::{methods::SendMessage};

use super::bot::BotContext;


#[handler(command = "/start")]
pub async fn start_handler(context: &BotContext, command: Command) -> Result<HandlerResult, ExecuteError> {
    let message = command.get_message();
    let chat_id = message.get_chat_id();
    info!("{} {} - {}", chat_id, message.get_chat_username().unwrap(), message.get_text().unwrap().data);

    let method = SendMessage::new(chat_id, "Use /time");
    let _result = context.api.execute(method).await;

    return Ok(HandlerResult::Stop)
}

#[handler(command = "/time")]
pub async fn command_handler(context: &BotContext, command: Command) -> Result<HandlerResult, ExecuteError> {
    let message = command.get_message();
    let chat_id = message.get_chat_id();
    info!("{} {} - {}", chat_id, message.get_chat_username().unwrap(), message.get_text().unwrap().data);

    let now = Utc::now();
    let formatted_utc = format!("{:02}:{:02}:{:02} {:02}.{:02}.{}", now.hour(), now.minute(), now.second(), now.day(), now.month(), now.year());

    let _mt = context.mt.lock().await;
    let formatted_mtc = format!("{:02}:{:02}:{:02}", _mt.get_sol_hour(), _mt.get_sol_minute(), _mt.get_sol_second());
    let mmt = format!("{:02}:{:02}:{:02}", _mt.get_hour(), _mt.get_minute(), _mt.get_second());

    let line = format!("```\nUTC: {utc}\nMSD: {msd}\n     Year {year}, Sol {sol}\nMTC: {mtc}\nMMT: {mmt}```",
                       utc=formatted_utc,
                       msd=_mt.get_msd(),
                       year=_mt.get_sol_year(),
                       sol=_mt.get_sol_day(),
                       mtc=formatted_mtc,
                       mmt=mmt);

    let method = SendMessage::new(chat_id, line).parse_mode(ParseMode::Markdown);
    let _result = context.api.execute(method).await;

    return Ok(HandlerResult::Stop)
}
