use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "spamer_groups")]
pub struct Model {
	#[sea_orm(primary_key, auto_increment = false)]
	pub id: i32, // ID в ресстре группировок спамеров
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
	#[sea_orm(has_many = "super::spamer::Entity")]
	Spamers,
}

impl Related<super::spamer::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::Spamers.def()
	}
}

impl ActiveModelBehavior for ActiveModel {}
