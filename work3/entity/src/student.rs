use serde::{Serialize,Deserialize};
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "student")]
pub struct Model {

    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: String,

    pub name: String,

    pub age: i32,

    pub class_id: String,

    pub club_id: String
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}