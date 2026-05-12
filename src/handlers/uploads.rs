use axum::{
    extract::{Multipart, Path, State},
    Json,
};

use crate::{
    error::{AppError, Result},
    models::{Player, PlayerPosition, PlayerRow},
    AppState,
};

const UPLOADS_DIR: &str = "uploads/avatars";
const MAX_SIZE: usize = 5 * 1024 * 1024; // 5 MB

pub async fn upload_avatar(
    State(pool): State<AppState>,
    Path(id): Path<i64>,
    mut multipart: Multipart,
) -> Result<Json<Player>> {
    let row = sqlx::query_as::<_, PlayerRow>("SELECT id, name, avatar FROM players WHERE id = ?")
        .bind(id)
        .fetch_optional(&pool)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Player {} not found", id)))?;

    while let Some(field) = multipart.next_field().await? {
        if field.name() != Some("avatar") {
            continue;
        }

        let content_type = field.content_type().unwrap_or("image/jpeg").to_string();
        let ext = match content_type.as_str() {
            "image/png" => "png",
            "image/gif" => "gif",
            "image/webp" => "webp",
            _ => "jpg",
        };

        let data = field.bytes().await?;
        if data.len() > MAX_SIZE {
            return Err(AppError::Internal("File too large (max 5 MB)".to_string()));
        }
        if data.is_empty() {
            return Err(AppError::Internal("Empty file".to_string()));
        }

        let ts = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis();
        let filename = format!("{}_{}.{}", id, ts, ext);
        let path = format!("{}/{}", UPLOADS_DIR, filename);

        tokio::fs::write(&path, &data).await?;

        if let Some(old) = &row.avatar {
            let old_path = format!("{}/{}", UPLOADS_DIR, old);
            let _ = tokio::fs::remove_file(&old_path).await;
        }

        sqlx::query("UPDATE players SET avatar = ? WHERE id = ?")
            .bind(&filename)
            .bind(id)
            .execute(&pool)
            .await?;

        let positions = sqlx::query_as::<_, PlayerPosition>(
            "SELECT id, player_id, position, pac, sho, pas, dri, def, phy, sort_order \
             FROM player_positions WHERE player_id = ? ORDER BY sort_order, id",
        )
        .bind(id)
        .fetch_all(&pool)
        .await?;

        return Ok(Json(Player {
            id: row.id,
            name: row.name,
            avatar: Some(filename),
            positions,
        }));
    }

    Err(AppError::Internal(
        "No 'avatar' field found in request".to_string(),
    ))
}
