use poise::serenity_prelude::*;
use shuttle_runtime::async_trait;

<<<<<<< HEAD
use crate::{api_connector::service::ApiConnectionService, builders, operations, graphql::queries::update_game_mutation::GameEditState};
=======
use crate::{api_connector::service::ApiConnectionService, builders, graphql::queries::{get_game_query, get_games_query::{self, GetGamesQueryGames}, int_to_game_result, update_game_mutation::{self, GameResult}}, operations};
>>>>>>> f5c5226 (registration logic)

pub struct MainEventHandler {
    api: ApiConnectionService
}

impl MainEventHandler {
    pub fn new(client: reqwest::Client) -> Self {
        MainEventHandler { api: ApiConnectionService::new(client) }
    }

<<<<<<< HEAD
    async fn dispatch_buttons(&self, context: &Context, interaction: &ComponentInteraction, component_id: &String, channel: u64, user: u64) -> Result<(), crate::Error> {
        match component_id.as_str() {
=======
    async fn dispatch_buttons(&self, context: &Context, interaction: &ComponentInteraction, id: &String, channel: u64, user: u64) -> Result<(), crate::Error> {
        match id.as_str() {
>>>>>>> f5c5226 (registration logic)
            "create_report_button" => {
                builders::report_message::initial_build(context, &self.api, &interaction, component_id, channel, user).await?;
            },
            "start_report" => {
                let response_message = builders::report_message::build_game_message(
                    context, &self.api, interaction.message.id.get()).await.unwrap();
                interaction.create_response(context, poise::serenity_prelude::CreateInteractionResponse::UpdateMessage(response_message)).await?;
            },
            "bargains_data_button" => {
                operations::report_creation::show_bargains_modal(interaction, context).await?;
            },
            "opponent_data_button" => {
                operations::report_creation::switch_to_edition_state(interaction, context, &self.api, GameEditState::OPPONENT_DATA).await?;
            },
            "player_data_button" => {
                operations::report_creation::switch_to_edition_state(interaction, context, &self.api, GameEditState::PLAYER_DATA).await?;
            }
            "result_data_button" => {
                operations::report_creation::switch_to_edition_state(interaction, context, &self.api, GameEditState::RESULT_DATA).await?;
            },
            "next_game_button" => {
                operations::report_creation::switch_games(interaction, context, &self.api, 1).await?;
            },
            "previous_game_button" => {
                operations::report_creation::switch_games(interaction, context, &self.api, -1).await?;
            },
            "submit_report" => {
<<<<<<< HEAD
                operations::report_creation::generate_final_report_message(interaction, context, &self.api).await?;
=======
                let message = interaction.message.id.get();
                let current_match = self.api.get_match(None, None, Some(message.to_string())).await.unwrap();
                if let Some(match_data) = current_match {
                    let tournament_data = self.api.get_tournament_data(Some(match_data.tournament), None, None).await.unwrap().unwrap();
                    let operator_data = self.api.get_operator_data(tournament_data.operator).await.unwrap();
                    let output_channel = ChannelId::from(operator_data.generated as u64);
                    let first_user = self.api.get_user(Some(match_data.first_player), None).await.unwrap().unwrap();
                    let participant = self.api.get_participant(tournament_data.id, first_user.id).await.unwrap().unwrap();
                    let second_user = self.api.get_user(Some(match_data.second_player.unwrap()), None).await.unwrap().unwrap().nickname;
                    let games = self.api.get_games(match_data.id).await.unwrap();
                    let sorted_games = games.iter()
                        .sorted_by_key(|g| g.number)
                        .collect::<Vec<&GetGamesQueryGames>>();

                    let first_player_wins = sorted_games.iter()
                        .filter(|g| g.result == get_games_query::GameResult::FIRST_PLAYER_WON)
                        .count();

                    let second_player_wins = sorted_games.iter()
                        .filter(|g| g.result == get_games_query::GameResult::SECOND_PLAYER_WON)
                        .count();

                    let mut fields = vec![];
                    for game in &sorted_games {
                        fields.push(
                            (
                                format!("_Игра {}_", game.number),
                                format!("**{},** _{}_ **{}** **{},** _{}_ [**{}**]", 
                                &self.api.races.iter().find(|r| r.id == game.first_player_race.unwrap()).unwrap().name,
                                &self.api.get_hero(game.first_player_hero.unwrap()).await.unwrap().unwrap().name,
                                match game.result {
                                    get_games_query::GameResult::FIRST_PLAYER_WON => ">".to_string(),
                                    get_games_query::GameResult::SECOND_PLAYER_WON => "<".to_string(),
                                    _=> "?".to_string()
                                },
                                &self.api.races.iter().find(|r| r.id == game.second_player_race.unwrap()).unwrap().name,
                                &self.api.get_hero(game.second_player_hero.unwrap()).await.unwrap().unwrap().name,
                                game.bargains_amount.to_string()
                                ),
                                false
                            ) 
                        )
                    }

                    fields.push(
                        (
                            "_Счёт_".to_string(),
                            format!("**{} - {}**", first_player_wins, second_player_wins),
                            false
                        )
                    );

                    let message_builder = CreateMessage::new()
                        .add_embed(
                            CreateEmbed::new()
                                .title(format!("**Турнир {}**, _групповой этап, группа_ **{}**", &tournament_data.name.to_uppercase(), participant.group))
                                .description(format!("**{}** _VS_ **{}**", &first_user.nickname, &second_user))
                                .fields(fields)
                        );
                    
                    output_channel.send_message(context, message_builder).await.unwrap();
                    interaction.create_response(context, CreateInteractionResponse::UpdateMessage(
                        CreateInteractionResponseMessage::new()
                            //.content("Отчет успешно создан, можете закрыть это сообщение.")
                            .add_embed(CreateEmbed::new().title("Отчет успешно создан, можете закрыть это сообщение."))
                            .components(vec![])
                    )).await.unwrap();
                    //context.http.delete_message(ChannelId::from(tournament_data.channel as u64), MessageId::from(interaction.message.id.get()), Some("Report cleanup")).await.unwrap();
                }
>>>>>>> f5c5226 (registration logic)
            },
            "register_user_button" => {
                operations::registration::try_register_in_tournament(interaction, context, &self.api).await?;
            },
            "unregister_user_button" => {
                operations::registration::try_remove_registration(interaction, context, &self.api).await?;
            },
            "update_user_data_button" => {
                operations::registration::try_update_user_data(interaction, context, &self.api).await?;
            }
            _=> {}
        }
        Ok(())
    }

    async fn dispatch_selection(&self, context: &Context, interaction: &ComponentInteraction, message_id: u64, component_id: &String, selected: &String) -> Result<(), crate::Error> {
        match component_id.as_str() {
            "games_count_selector" => {
                operations::report_creation::select_games_count(interaction, context, &self.api, message_id, selected).await?;
            },
            "opponent_selector" => {
                operations::report_creation::select_opponent(interaction, context, &self.api, message_id, selected).await?;
            },
            "player_race_selector" => {
                operations::report_creation::select_player_race(interaction, context, &self.api, message_id, selected).await?;
            },
            "opponent_race_selector" => {
                operations::report_creation::select_opponent_race(interaction, context, &self.api, message_id, selected).await?;
            },
            "player_hero_selector" => {
                operations::report_creation::select_player_hero(interaction, context, &self.api, message_id, selected).await?;
            },
            "opponent_hero_selector" => {
                operations::report_creation::select_opponent_hero(interaction, context, &self.api, message_id, selected).await?;
            },
            "game_result_selector" => {
                operations::report_creation::select_game_result(interaction, context, &self.api, message_id, selected).await?;
            }
            _=> {}
        }
        Ok(())
    }

    async fn dispatch_modals(&self, context: &Context, interaction: &ModalInteraction) -> Result<(), crate::Error> {
        match interaction.data.custom_id.as_str() {
            "player_data_modal" => {
                operations::report_creation::process_bargains_modal(interaction, context, &self.api).await?;
            },
            "user_lobby_nickname_modal" => {
                operations::registration::process_registration_modal(interaction, &context, &self.api).await?;
            },
            "user_update_nickname_modal" => {
                operations::registration::process_user_update_modal(interaction, &context, &self.api).await?;
            }
            _=> {}
        }
        Ok(())
    }

    async fn dispatch_message_created_by_interaction(&self, _context: &Context, message_id: u64, interaction_id: u64) -> Result<(), crate::Error> {
        operations::report_creation::save_report_user_message(_context, &self.api, message_id, interaction_id).await?;
        Ok(())
    }
}

#[async_trait]
impl EventHandler for MainEventHandler {
    async fn interaction_create(&self, context: Context, interaction: Interaction) {
        if let Some(component_interaction) =  interaction.as_message_component() {
            let channel = component_interaction.channel_id;
            let user = &component_interaction.user;
            tracing::info!("Component interaction in channel {} by user {}", channel.name(&context.http).await.unwrap(), user.name);

            match component_interaction.data.kind {
                ComponentInteractionDataKind::Button => {
                    let id = &component_interaction.data.custom_id;
                    self.dispatch_buttons(&context, &component_interaction, id, channel.get(), user.id.get()).await.unwrap();
                },
                ComponentInteractionDataKind::StringSelect { ref values } => {
                    let id = &component_interaction.data.custom_id;
                    let selected_value = values.first();
                    let message = component_interaction.message.id.get();
                    self.dispatch_selection(&context, &component_interaction, message, id, selected_value.unwrap()).await.unwrap();
                },
                _=> {}
            }
        }
        else if let Some(modal_interaction) = interaction.as_modal_submit() {
<<<<<<< HEAD
            self.dispatch_modals(&context, modal_interaction).await.unwrap();
=======
            match modal_interaction.data.custom_id.as_str() {
                "player_data_modal" => {
                    let message = &modal_interaction.message.as_ref().unwrap().content;
                    let mut bargains_value = 0;
                    tracing::info!("Modal was created from message: {}", message);
                    tracing::info!("Modal data: {:?}", &modal_interaction.data);
                    for row in &modal_interaction.data.components {
                        for component in &row.components {
                            match component {
                                ActionRowComponent::InputText(text) => {
                                    if text.custom_id.as_str() == "bargains_amount_input" {
                                        let value = i32::from_str_radix(&text.value.as_ref().unwrap(), 10).unwrap();
                                        bargains_value = value;
                                        tracing::info!("Bargains amount: {}", value);
                                    }
                                },
                                _=> {}
                            }
                        }
                    }
                    let match_data = self.api.get_match(
                        None, 
                        None, 
                        Some(modal_interaction.message.as_ref().unwrap().id.get().to_string())
                    ).await.unwrap();
                    if let Some(existing_match) = match_data {
                        self.api.update_game(
                            existing_match.id, 
                            existing_match.current_game, 
                            None, 
                            None,
                            None, 
                            None,
                            None,
                            Some(bargains_value as i64),
                            None
                        ).await.unwrap();
                        let rebuilt_message=  builders::report_message::build_game_message(&context, &self.api, modal_interaction.message.as_ref().unwrap().id.get()).await.unwrap();
                        modal_interaction.create_response(context, CreateInteractionResponse::UpdateMessage(rebuilt_message)).await.unwrap();
                    }
                },
                "user_lobby_nickname_modal" => {
                    operations::registration::process_registration_modal(modal_interaction, &context, &self.api).await.unwrap();
                },
                "user_update_nickname_modal" => {
                    operations::registration::process_user_update_modal(modal_interaction, &context, &self.api).await.unwrap();
                }
                _=> {}
            }
>>>>>>> f5c5226 (registration logic)
        }
    }

    async fn message_delete(&self, context: Context, channel_id: ChannelId, deleted_message_id: MessageId, _guild_id: Option<GuildId>) {
        tracing::info!("Message {} was deleted from channel {}", deleted_message_id.get(), &channel_id.name(context).await.unwrap());
    }

    async fn message(&self, context: Context, new_message: Message) {
        if let Some(interaction) = new_message.interaction_metadata {
            match *interaction {
                MessageInteractionMetadata::Component(component_interaction) => {
                    let id = component_interaction.id.get();
                    tracing::info!("Message {} created as response to interaction {}", new_message.id.get(), id);
                    self.dispatch_message_created_by_interaction(&context, new_message.id.get(), id).await.unwrap();
                    
                },
                _=> {}
            }
        }
    }
}