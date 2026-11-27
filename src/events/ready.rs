use poise::serenity_prelude as serenity;
use serenity::all::{ActivityData, OnlineStatus};
use crate::types::Error;

pub async fn handle_ready(
    ctx: &serenity::Context,
    ready: &serenity::Ready,
) -> Result<(), Error> {
    tracing::info!("Successfully logged in as {}!", ready.user.name);

    let activity = ActivityData::playing("https://tilt.ninja");
    let activity = ActivityData {
        state: Some("with Tilt".to_string()),
        ..activity
    };
    ctx.set_presence(Some(activity), OnlineStatus::Idle);

    Ok(())
}
