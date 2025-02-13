use sea_orm::prelude::*;

pub type TournamentModel = Model;

#[derive(Debug, EnumIter, DeriveActiveEnum, Clone, Copy, PartialEq, Eq, async_graphql::Enum)]
#[sea_orm(rs_type = "i32", db_type = "Integer")]
pub enum TournamentStage {
    Unknown = 0,
    GroupStage = 1,
    PlayOff = 2
}

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "tournaments_new")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub operator_id: Uuid,
    pub channel_id: i64,
    pub name: String,
    pub stage: TournamentStage,
    pub register_channel: i64,
    pub with_bargains: bool,
    pub with_foreign_heroes: bool,
    pub role_id: i64
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

#[async_graphql::Object]
impl TournamentModel {
    async fn id(&self) -> Uuid {
        self.id
    }

    async fn operator(&self) -> Uuid {
        self.operator_id
    }

    async fn channel(&self) -> i64 {
        self.channel_id
    }

    async fn name(&self) -> String {
        self.name.clone()
    }

    async fn stage(&self) -> TournamentStage {
        self.stage
    }

    async fn register_channel(&self) -> i64 {
        self.register_channel
    }

    async fn with_bargains(&self) -> bool {
        self.with_bargains
    }

    async fn with_foreign_heroes(&self) -> bool {
        self.with_foreign_heroes
    }

    async fn role(&self) -> i64 {
        self.role_id
    }
}