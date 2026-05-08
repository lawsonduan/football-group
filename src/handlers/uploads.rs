use axum::{
    extract::{Multipart, Path, State},
    Json,
};

use crate::{
    error::{AppError, Result},
    models::Player,
    AppState,
};

const UPLOADS_DIR: &str = "uploads/avatars";
const MAX_SIZE: usize = 5 * 1024 * 1024; // 5 MB

pub async fn upload_avatar(
    State(pool): State<AppState>,
    Path(id): Path<i64>,
    mut multipart: Multipart,
) -> Result<Json<Player>> {
    // Verify player exists
    let player = sqlx::query_as::<_, Player>(
        "SELECT id, name, position, avatar FROM players WHERE id = ?",
    )
    .bind(id)
    .fetch_optional(&pool)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("Player {} not found", id)))?;

    while let Some(field) = multipart.next_field().await? {
        if field.name() != Some("avatar") {
            continue;
        }

        let content_type = field
            .content_type()
            .unwrap_or("image/jpeg")
            .to_string();

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

        // Generate a unique filename using player id + timestamp
        let ts = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis();
        let filename = format!("{}_{}.{}", id, ts, ext);
        let path = format!("{}/{}", UPLOADS_DIR, filename);

        tokio::fs::write(&path, &data).await?;

        // Delete old avatar file if any
        if let Some(old) = &player.avatar {
            let old_path = format!("{}/{}", UPLOADS_DIR, old);
            let _ = tokio::fs::remove_file(&old_path).await; // ignore errors
        }

        // Persist new filename
        sqlx::query("UPDATE players SET avatar = ? WHERE id = ?")
            .bind(&filename)
            .bind(id)
            .execute(&pool)
            .await?;

        return Ok(Json(Player {
            id: player.id,
            name: player.name,
            position: player.position,
            avatar: Some(filename),
            pac: player.pac,
            sho: player.sho,
            pas: player.pas,
            dri: player.dri,
            def: player.def,
            phy: player.phy,
        }));
    }

    Err(AppError::Internal(
        "No 'avatar' field found in request".to_string(),
    ))
}
