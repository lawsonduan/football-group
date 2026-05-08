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

const SELECT: &str =
    "SELECT id, name, position, avatar, pac, sho, pas, dri, def, phy FROM players";

pub async fn list_players(State(pool): State<AppState>) -> Result<Json<Vec<Player>>> {
    let players = sqlx::query_as::<_, Player>(&format!("{SELECT} ORDER BY id"))
        .fetch_all(&pool)
        .await?;

    Ok(Json(players))
}

pub async fn create_player(
    State(pool): State<AppState>,
    Json(payload): Json<CreatePlayer>,
) -> Result<(StatusCode, Json<Player>)> {
    let position = payload.position.unwrap_or_else(|| "none".to_string());
    let pac = payload.pac.unwrap_or(50);
    let sho = payload.sho.unwrap_or(50);
    let pas = payload.pas.unwrap_or(50);
    let dri = payload.dri.unwrap_or(50);
    let def = payload.def.unwrap_or(50);
    let phy = payload.phy.unwrap_or(50);

    let id = sqlx::query(
        "INSERT INTO players (name, position, pac, sho, pas, dri, def, phy) \
         VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(&payload.name)
    .bind(&position)
    .bind(pac)
    .bind(sho)
    .bind(pas)
    .bind(dri)
    .bind(def)
    .bind(phy)
    .execute(&pool)
    .await?
    .last_insert_rowid();

    Ok((
        StatusCode::CREATED,
        Json(Player { id, name: payload.name, position, avatar: None, pac, sho, pas, dri, def, phy }),
    ))
}

pub async fn update_player(
    State(pool): State<AppState>,
    Path(id): Path<i64>,
    Json(payload): Json<UpdatePlayer>,
) -> Result<Json<Player>> {
    let e = sqlx::query_as::<_, Player>(&format!("{SELECT} WHERE id = ?"))
        .bind(id)
        .fetch_optional(&pool)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Player {} not found", id)))?;

    let name = payload.name.unwrap_or(e.name);
    let position = payload.position.unwrap_or(e.position);
    let pac = payload.pac.unwrap_or(e.pac);
    let sho = payload.sho.unwrap_or(e.sho);
    let pas = payload.pas.unwrap_or(e.pas);
    let dri = payload.dri.unwrap_or(e.dri);
    let def = payload.def.unwrap_or(e.def);
    let phy = payload.phy.unwrap_or(e.phy);

    sqlx::query(
        "UPDATE players SET name=?, position=?, pac=?, sho=?, pas=?, dri=?, def=?, phy=? \
         WHERE id=?",
    )
    .bind(&name)
    .bind(&position)
    .bind(pac)
    .bind(sho)
    .bind(pas)
    .bind(dri)
    .bind(def)
    .bind(phy)
    .bind(id)
    .execute(&pool)
    .await?;

    Ok(Json(Player { id, name, position, avatar: e.avatar, pac, sho, pas, dri, def, phy }))
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
