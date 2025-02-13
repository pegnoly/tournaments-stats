use anyhow::Context as _;
use api_connector::service::ApiConnectionService;
use event_handler::MainEventHandler;
use parser::service::ParserService;
use poise::serenity_prelude::{ClientBuilder, EventHandler, GatewayIntents, Integration, Interaction};
use shuttle_runtime::{async_trait, SecretStore};
use shuttle_serenity::ShuttleSerenity;
use tokio::sync::RwLock;

pub mod commands;
pub mod parser;
pub mod api_connector;
pub mod graphql;
pub mod builders;
pub mod event_handler;
pub mod operations;
pub mod types;

pub struct Data {
    pub api_connection_service: ApiConnectionService,
    pub parser_service: ParserService
} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[shuttle_runtime::main]
async fn main(#[shuttle_runtime::Secrets] secret_store: SecretStore) -> ShuttleSerenity {
    // Get the discord token set in `Secrets.toml`
    let discord_token = secret_store
        .get("DISCORD_TOKEN")
        .context("'DISCORD_TOKEN' was not found")?;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                commands::init_tournament(),
                commands::parse_results(),
                commands::init_services(),
                commands::create_user(),
                commands::setup_tournament(),
                commands::delete_unused(),
                commands::register_in_tournament()
            ],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {
                    api_connection_service: ApiConnectionService::new(reqwest::Client::new()),
                    parser_service: ParserService {}
                })
            })
        })
        .build();

    let client = ClientBuilder::new(discord_token, GatewayIntents::all())
        .framework(framework)
        .event_handler(MainEventHandler::new(reqwest::Client::new()))
        .await
        .map_err(shuttle_runtime::CustomError::new)?;

    Ok(client.into())
}
