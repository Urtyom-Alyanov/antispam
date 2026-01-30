use sea_orm::DatabaseConnection;

use crate::flood_state::FloodState;

#[derive(thiserror::Error, Debug)]
pub enum AppError {
	#[error("Database error")]
	Db(#[from] sea_orm::DbErr),
	#[error("Network error")]
	Net(#[from] reqwest::Error),
}

pub struct AppState {
	pub db: DatabaseConnection,
	pub protector: FloodState,
}
