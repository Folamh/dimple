use poise::serenity_prelude::{self as serenity, CacheHttp};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;
struct Data {}

#[poise::command(prefix_command)]
async fn register(ctx: Context<'_>) -> Result<(), Error> {
    poise::builtins::register_application_commands_buttons(ctx).await?;
    Ok(())
}

// Create a poll for seleting a date
#[poise::command(slash_command)]
async fn schedule(
    ctx: Context<'_>,
    #[description = "Begining date"] arg: String,
) -> Result<(), Error> {
    let start_date = chrono::NaiveDate::parse_from_str(&arg, "%Y-%m-%d")?;
    let end_date = start_date + chrono::Duration::days(7);
    Ok(())
}

async fn event_listener(
    _ctx: &serenity::Context,
    event: &poise::Event<'_>,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    _user_data: &Data,
) -> Result<(), Error> {
    match event {
        poise::Event::Ready { data_about_bot } => {
            println!("{} is connected!", data_about_bot.user.name)
        }
        poise::Event::ReactionAdd { add_reaction } => {
            println!("Added: {}", &add_reaction.emoji.as_data());
            if add_reaction.emoji.unicode_eq("📌") {
                let msg = add_reaction.message(&_ctx.http).await?;
                msg.pin(&_ctx.http()).await?;
            }
        }
        poise::Event::ReactionRemove { removed_reaction } => {
            println!("Removed: {}", &removed_reaction.emoji.as_data());
            if removed_reaction.emoji.unicode_eq("📌") {
                let msg = removed_reaction.message(&_ctx.http).await?;
                msg.unpin(&_ctx.http()).await?;
            }
        }
        _ => {}
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![register()],
            event_handler: |_ctx, event, _framework, _data| {
                Box::pin(event_listener(_ctx, event, _framework, _data))
            },
            ..Default::default()
        })
        .token(std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN"))
        .intents(
            serenity::GatewayIntents::non_privileged()
                | serenity::GatewayIntents::MESSAGE_CONTENT
                | serenity::GatewayIntents::GUILD_MESSAGE_REACTIONS,
        )
        .setup(move |_ctx, _ready, _framework| Box::pin(async move { Ok(Data {}) }));

    framework.run().await.unwrap();
}
