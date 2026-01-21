use std::{
	collections::{HashMap, HashSet},
	sync::{Arc, Mutex},
};

use chrono::{DateTime, Duration, Utc};

#[derive(Debug, Clone, Copy)]
pub enum ContentType {
	Message,
	WallComment,
	WallPost,
}

#[derive(Debug, Clone)]
pub struct MessageInfo {
	pub id: i64,
	pub timestamp: DateTime<Utc>,
	pub context_id: Option<i64>, // Идентификатор владельца контекста (пост, сообщество и тд.)
	pub content_type: ContentType,
}

pub struct FloodState {
	history: Arc<Mutex<HashMap<i64, Vec<MessageInfo>>>>,
	processing: Arc<Mutex<HashSet<i64>>>,
}

impl FloodState {
	pub fn new() -> Self {
		Self {
			history: Arc::new(Mutex::new(HashMap::new())),
			processing: Arc::new(Mutex::new(HashSet::new())),
		}
	}

	pub fn check_and_push_msgs(
		&self,
		user_id: i64,
		msg_id: i64,
		content_type: ContentType,
		context_id: Option<i64>,
		limit_duration: Duration,
		limit_count: usize,
	) -> bool {
		let now = Utc::now();

		let mut history = self.history.lock().unwrap(); // Инициализируем мьютекс для доступа к истории сообщений
		let messages = history.entry(user_id).or_insert(Vec::new()); // Инициализируем вектор сообщений для пользователя

		messages.push(MessageInfo {
			id: msg_id,
			timestamp: now,
			context_id: context_id,
			content_type: content_type,
		});

		messages.retain(|m| now - m.timestamp <= limit_duration); // Удаляем сообщения старше заданного времени

		if messages.len() >= limit_count {
			true // Возвращаем идентификаторы сообщений
		} else {
			false
		}
	}

	pub fn get_history(&self, user_id: i64) -> Vec<MessageInfo> {
		let history = self.history.lock().unwrap(); // Инициализируем мьютекс для доступа к истории сообщений
		history.get(&user_id).cloned().unwrap_or_default()
	}

	pub fn is_processing(&self, user_id: i64) -> bool {
		self.processing.lock().unwrap().contains(&user_id)
	}

	pub fn set_processing(&self, user_id: i64, status: bool) {
		let mut p = self.processing.lock().unwrap();
		if status {
			p.insert(user_id);
		} else {
			p.remove(&user_id);
		}
	}

	// Вызов после того как всё отработало ГУБАМИ
	pub fn clear_history(&self, user_id: i64) {
		let mut history = self.history.lock().unwrap(); // Инициализируем мьютекс для доступа к истории сообщений
		history.remove(&user_id); // Удаляем историю сообщений пользователя
	}
}
