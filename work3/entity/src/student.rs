use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug,Serialize,Deserialize,DeriveEntityModel,Clone)]
#[sea_orm(table_name = "student")]
pub struct Model {

    #[sea_orm(primary_key)]
    pub id: String,

    pub name: String,

    pub age: i32,

    pub class_id: String,

    pub club_id: String
}

#[derive(Copy,Clone,Debug,EnumIter,DeriveRelation)]
pub enum Relation{}

impl ActiveModelBehavior for ActiveModel{}