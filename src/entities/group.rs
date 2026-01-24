use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "groups")]
pub struct Model {
	#[sea_orm(primary_key, auto_increment = false)]
	pub id: u64, // ID группы ВК
	pub token: String,              // Токен
	pub user_token: Option<String>, // Токен пользователя
	pub limit_count: Option<i32>,   // Лимит сообщений
	pub limit_secs: Option<i32>,    // Длительность окна (в сек)
	pub is_active: bool,
	pub confirmation_token: String, // Токен подтверждения
	pub secret: Option<String>,     // Секретная строка
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
