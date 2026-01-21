use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "spamers")]
pub struct Model {
	#[sea_orm(primary_key, auto_increment = false)]
	pub id: i64, // ID пользователя ВК
	pub spamer_group_id: i32, // ID в ресстре группировок спамеров
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
	#[sea_orm(
		belongs_to = "super::spamer_group::Entity",
		from = "Column::SpamerGroupId",
		to = "super::spamer_group::Column::Id"
	)]
	SpamerGroup,
}

impl Related<super::spamer_group::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::SpamerGroup.def()
	}
}

impl ActiveModelBehavior for ActiveModel {}
