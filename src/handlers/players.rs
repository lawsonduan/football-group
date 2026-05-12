use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use std::collections::HashMap;

use crate::{
    error::{AppError, Result},
    models::{CreatePlayer, Player, PlayerPosition, PlayerRow, PositionInput, UpdatePlayer},
    AppState,
};

/// Fetch a single `Player` (with all positions) by id.
async fn fetch_player(pool: &AppState, id: i64) -> Result<Player> {
    let row = sqlx::query_as::<_, PlayerRow>("SELECT id, name, avatar FROM players WHERE id = ?")
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Player {} not found", id)))?;

    let positions = sqlx::query_as::<_, PlayerPosition>(
        "SELECT id, player_id, position, pac, sho, pas, dri, def, phy, sort_order \
         FROM player_positions WHERE player_id = ? ORDER BY sort_order, id",
    )
    .bind(id)
    .fetch_all(pool)
    .await?;

    Ok(Player { id: row.id, name: row.name, avatar: row.avatar, positions })
}

/// Insert position records for a player (helper used by create/update).
async fn insert_positions(
    pool: &AppState,
    player_id: i64,
    inputs: &[PositionInput],
) -> Result<()> {
    for (i, p) in inputs.iter().enumerate() {
        sqlx::query(
            "INSERT INTO player_positions \
             (player_id, position, pac, sho, pas, dri, def, phy, sort_order) \
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(player_id)
        .bind(&p.position)
        .bind(p.pac.unwrap_or(50))
        .bind(p.sho.unwrap_or(50))
        .bind(p.pas.unwrap_or(50))
        .bind(p.dri.unwrap_or(50))
        .bind(p.def.unwrap_or(50))
        .bind(p.phy.unwrap_or(50))
        .bind(i as i64)
        .execute(pool)
        .await?;
    }
    Ok(())
}

pub async fn list_players(State(pool): State<AppState>) -> Result<Json<Vec<Player>>> {
    let rows = sqlx::query_as::<_, PlayerRow>("SELECT id, name, avatar FROM players ORDER BY id")
        .fetch_all(&pool)
        .await?;

    let all_positions = sqlx::query_as::<_, PlayerPosition>(
        "SELECT id, player_id, position, pac, sho, pas, dri, def, phy, sort_order \
         FROM player_positions ORDER BY player_id, sort_order, id",
    )
    .fetch_all(&pool)
    .await?;

    let mut pos_map: HashMap<i64, Vec<PlayerPosition>> = HashMap::new();
    for p in all_positions {
        pos_map.entry(p.player_id).or_default().push(p);
    }

    let players = rows
        .into_iter()
        .map(|r| Player {
            id: r.id,
            positions: pos_map.remove(&r.id).unwrap_or_default(),
            name: r.name,
            avatar: r.avatar,
        })
        .collect();

    Ok(Json(players))
}

pub async fn create_player(
    State(pool): State<AppState>,
    Json(payload): Json<CreatePlayer>,
) -> Result<(StatusCode, Json<Player>)> {
    let id = sqlx::query("INSERT INTO players (name) VALUES (?)")
        .bind(&payload.name)
        .execute(&pool)
        .await?
        .last_insert_rowid();

    insert_positions(&pool, id, &payload.positions).await?;

    Ok((StatusCode::CREATED, Json(fetch_player(&pool, id).await?)))
}

pub async fn update_player(
    State(pool): State<AppState>,
    Path(id): Path<i64>,
    Json(payload): Json<UpdatePlayer>,
) -> Result<Json<Player>> {
    let existing = fetch_player(&pool, id).await?;
    let name = payload.name.unwrap_or(existing.name);

    sqlx::query("UPDATE players SET name = ? WHERE id = ?")
        .bind(&name)
        .bind(id)
        .execute(&pool)
        .await?;

    if let Some(positions) = payload.positions {
        sqlx::query("DELETE FROM player_positions WHERE player_id = ?")
            .bind(id)
            .execute(&pool)
            .await?;
        insert_positions(&pool, id, &positions).await?;
    }

    Ok(Json(fetch_player(&pool, id).await?))
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
