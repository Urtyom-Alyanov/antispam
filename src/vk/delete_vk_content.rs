use crate::flood_state::{ContentType, MessageInfo};

pub async fn delete_vk_content(
    token: &str, // Токен пользователя с правом wall
    msg: MessageInfo,
) -> Result<(), reqwest::Error> {
    let client = reqwest::Client::new();

    let method = match msg.content_type {
        ContentType::WallComment => "wall.deleteComment",
        ContentType::Message => "messages.delete", // Для сообщений нужны другие параметры
        ContentType::WallPost => "wall.delete",
    };

    if method == "messages.delete" {
        // Потом реализую
        return Ok(());
    }

    let mut params = vec![
        ("access_token", token.to_owned()),
        ("v", "5.199".to_owned()),
    ];

    if let ContentType::WallComment = msg.content_type {
        params.push(("comment_id", msg.id.to_string()));
        if let Some(owner_id) = msg.context_id {
            // Идентификатор владельца стены.
            params.push(("owner_id", owner_id.to_string()));
        }
    }

    if let ContentType::WallPost = msg.content_type {
        params.push(("post_id", msg.id.to_string()));
        if let Some(owner_id) = msg.context_id {
            // Идентификатор владельца стены.
            params.push(("owner_id", owner_id.to_string()));
        }
    }

    client
        .post(format!("https://api.vk.com/method/{}", method))
        .json(&params)
        .send()
        .await?;

    Ok(())
}
