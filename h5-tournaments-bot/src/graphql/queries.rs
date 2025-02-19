use get_match_query::GetMatchQueryTournamentMatch;
use get_tournament_query::GetTournamentQueryTournament;
use get_users_query::GetUsersQueryUsers;
use graphql_client::GraphQLQuery;

type UUID = uuid::Uuid;

#[derive(Debug, PartialEq, Eq)]
#[repr(i16)]
pub enum GameEditState {
    NotSelected = 0,
    PlayerData = 1,
    OpponentData = 2,
    ResultData = 3
}

#[derive(Debug, PartialEq, Eq)]
#[repr(i32)]
pub enum GameResult {
    NotSelected = 0,
    FirstPlayerWon = 1,
    SecondPlayerWon = 2
}

#[derive(Debug, PartialEq, Eq)]
#[repr(i32)]
pub enum TournamentEditState {
    NotSelected = 0,
    ChannelsData = 1,
    ReportsData = 2
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/queries/create_user.graphql",
    response_derives = "Debug"
)]
pub struct CreateUserMutation;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/queries/create_tournament.graphql",
    response_derives = "Debug"
)]
pub struct CreateTournamentMutation;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/queries/get_operator_section.graphql",
    response_derives = "Debug"
)]
pub struct GetOperatorSectionQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/queries/get_operator_data.graphql",
    response_derives = "Debug"
)]
pub struct GetOperatorDataQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/queries/get_tournament.graphql",
    response_derives = "Debug"
)]
pub struct GetTournamentQuery;
pub type GetTournamentResult = GetTournamentQueryTournament;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/queries/get_user.graphql",
    response_derives = "Debug"
)]
pub struct GetUserQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/queries/create_match.graphql",
    response_derives = "Debug"
)]
pub struct CreateMatchMutation;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/queries/update_match.graphql",
    response_derives = "Debug"
)]
pub struct UpdateMatchMutation;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/queries/get_match.graphql",
    response_derives = "Debug"
)]
pub struct GetMatchQuery;

pub type GetMatchResult = GetMatchQueryTournamentMatch;


#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/queries/get_users.graphql",
    response_derives = "Debug"
)]
pub struct GetUsersQuery;

pub type GetUsersResult = GetUsersQueryUsers;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/queries/update_user.graphql",
    response_derives = "Debug"
)]
pub struct UpdateUser;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/queries/create_game.graphql",
    response_derives = "Debug, PartialEq, Eq"
)]
pub struct CreateGameMutation;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/queries/update_game.graphql",
    response_derives = "Debug, PartialEq, Eq"
)]
pub struct UpdateGameMutation;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/queries/get_game.graphql",
    response_derives = "Debug, PartialEq, Eq"
)]
pub struct GetGameQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/queries/get_heroes.graphql",
    response_derives = "Debug"
)]
pub struct GetHeroesQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/queries/get_hero.graphql",
    response_derives = "Debug"
)]
pub struct GetHeroQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/queries/get_games.graphql",
    response_derives = "Debug, PartialEq, Eq"
)]
pub struct GetGamesQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/queries/get_participant.graphql",
    response_derives = "Debug"
)]
pub struct GetParticipant;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/queries/get_participants.graphql",
    response_derives = "Debug"
)]
pub struct GetParticipants;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/queries/create_participant.graphql",
    response_derives = "Debug"
)]
pub struct CreateParticipant;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/queries/delete_participant.graphql",
    response_derives = "Debug"
)]
pub struct DeleteParticipant;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/queries/create_organizer.graphql",
    response_derives = "Debug"
)]
pub struct CreateOrganizer;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/queries/create_tournament_builder.graphql",
    response_derives = "Debug"
)]
pub struct CreateTournamentBuilder;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/queries/get_organizer.graphql",
    response_derives = "Debug"
)]
pub struct GetOrganizer;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/queries/get_tournament_builder.graphql",
    response_derives = "Debug, PartialEq, Eq"
)]
pub struct GetTournamentBuilder;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/queries/update_tournament_builder.graphql",
    response_derives = "Debug, PartialEq, Eq"
)]
pub struct UpdateTournamentBuilder;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/queries/get_tournaments.graphql",
    response_derives = "Debug"
)]
pub struct GetTournaments;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/queries/update_tournament.graphql",
    response_derives = "Debug"
)]
pub struct UpdateTournament;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/queries/get_tournament_users.graphql",
    response_derives = "Debug"
)]
pub struct GetTournamentUsers;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/queries/update_participants_bulk.graphql",
    response_derives = "Debug"
)]
pub struct UpdateParticipantsBulk;

pub fn int_to_game_result(num: i32) -> update_game_mutation::GameResult {
    match num {
        1 => update_game_mutation::GameResult::FIRST_PLAYER_WON,
        2 => update_game_mutation::GameResult::SECOND_PLAYER_WON,
        _=> update_game_mutation::GameResult::NOT_SELECTED
    }
}

impl Into<update_tournament_builder::TournamentEditState> for get_tournament_builder::TournamentEditState {
    fn into(self) -> update_tournament_builder::TournamentEditState {
        match self {
            get_tournament_builder::TournamentEditState::CHANNELS_DATA => update_tournament_builder::TournamentEditState::CHANNELS_DATA,
            get_tournament_builder::TournamentEditState::NOT_SELECTED => update_tournament_builder::TournamentEditState::NOT_SELECTED,
            get_tournament_builder::TournamentEditState::REPORTS_DATA => update_tournament_builder::TournamentEditState::REPORTS_DATA,
            _=> update_tournament_builder::TournamentEditState::NOT_SELECTED
        }
    }
}