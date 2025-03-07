use sea_orm::prelude::*;
use serde::{Serialize, Deserialize};

pub type UserModel = Model;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "users")] 
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub discord_id: i64,
    pub discord_nick: String,
    pub nickname: String,
    pub registered_manually: bool
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Participant
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Participant => Entity::has_one(super::participant::Entity).into()
        }
    }
}

impl Related<super::participant::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Participant.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[async_graphql::Object]
impl UserModel {
    async fn id(&self) -> Uuid {
        self.id
    }

    async fn discord_id(&self) -> i64 {
        self.discord_id
    }

    async fn discord_nick(&self) -> String {
        self.discord_nick.clone()
    }

    async fn nickname(&self) -> String {
        self.nickname.clone()
    }

    async fn registered(&self) -> bool {
        self.registered_manually
    }
}

#[derive(Debug, Serialize, Deserialize, async_graphql::InputObject)]
pub struct UserBulkUpdatePayload {
    pub id: Uuid,
    pub discord_nick: Option<String>
}