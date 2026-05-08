use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

use crate::{
    error::{AppError, Result},
    models::{CreatePlayer, Player, UpdatePlayer},
    AppState,
};

const SELECT: &str = "SELECT id, name, position, avatar FROM players";

pub async fn list_players(State(pool): State<AppState>) -> Result<Json<Vec<Player>>> {
    let players =
        sqlx::query_as::<_, Player>(&format!("{SELECT} ORDER BY id"))
            .fetch_all(&pool)
            .await?;

    Ok(Json(players))
}

pub async fn create_player(
    State(pool): State<AppState>,
    Json(payload): Json<CreatePlayer>,
) -> Result<(StatusCode, Json<Player>)> {
    let position = payload.position.unwrap_or_else(|| "none".to_string());

    let id = sqlx::query("INSERT INTO players (name, position) VALUES (?, ?)")
        .bind(&payload.name)
        .bind(&position)
        .execute(&pool)
        .await?
        .last_insert_rowid();

    Ok((
        StatusCode::CREATED,
        Json(Player { id, name: payload.name, position, avatar: None }),
    ))
}

pub async fn update_player(
    State(pool): State<AppState>,
    Path(id): Path<i64>,
    Json(payload): Json<UpdatePlayer>,
) -> Result<Json<Player>> {
    let existing =
        sqlx::query_as::<_, Player>(&format!("{SELECT} WHERE id = ?"))
            .bind(id)
            .fetch_optional(&pool)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Player {} not found", id)))?;

    let name = payload.name.unwrap_or(existing.name);
    let position = payload.position.unwrap_or(existing.position);

    sqlx::query("UPDATE players SET name = ?, position = ? WHERE id = ?")
        .bind(&name)
        .bind(&position)
        .bind(id)
        .execute(&pool)
        .await?;

    Ok(Json(Player { id, name, position, avatar: existing.avatar }))
}

pub async fn delete_player(
    State(pool): State<AppState>,
    Path(id): Path<i64>,
) -> Result<StatusCode> {
    let result = sqlx::query("DELETE FROM players WHERE id = ?")
        .bind(id)
        .execute(&pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound(format!("Player {} not found", id)));
    }

    Ok(StatusCode::NO_CONTENT)
}
