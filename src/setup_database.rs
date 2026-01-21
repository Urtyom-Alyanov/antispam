use sea_orm::{ActiveValue::Set, Database, DatabaseConnection, EntityTrait, sea_query::OnConflict};

use crate::entities::setting;

pub async fn setup_database() -> DatabaseConnection {
	let db = Database::connect("sqlite://fast_antiflood_database.sqlite?mode=rwc")
		.await
		.expect("Не удалось подключиться к БД");

	let default_settings = vec![
		setting::ActiveModel {
			id: Set("LIMIT_COUNT".to_owned()),
			value: Set("20".to_owned()),
		},
		setting::ActiveModel {
			id: Set("LIMIT_SECONDS".to_owned()),
			value: Set((60 * 5).to_string()),
		},
		setting::ActiveModel {
			id: Set("MIN_DELAY_SECONDS".to_owned()),
			value: Set((5).to_string()),
		},
		setting::ActiveModel {
			id: Set("MAX_DELAY_SECONDS".to_owned()),
			value: Set((5).to_string()),
		},
		setting::ActiveModel {
			id: Set("USER_TOKEN".to_owned()),
			value: Set("".to_owned()),
		},
	];

	setting::Entity::insert_many(default_settings)
		.on_conflict(
			OnConflict::column(setting::Column::Id)
				.do_nothing()
				.to_owned(),
		)
		.exec(&db)
		.await
		.expect("Не удалось создать записи первоначальных настроек в БД");

	db
}
