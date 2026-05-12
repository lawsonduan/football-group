use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

use crate::{
    error::{AppError, Result},
    models::{CreateGame, Game, GameDetail, GamePlayer, UpdateGame},
    AppState,
};

pub async fn list_games(State(pool): State<AppState>) -> Result<Json<Vec<Game>>> {
    let games = sqlx::query_as::<_, Game>(
        "SELECT id, game_date, team_a_score, team_b_score, notes, created_at
         FROM games ORDER BY game_date DESC, created_at DESC",
    )
    .fetch_all(&pool)
    .await?;

    Ok(Json(games))
}

pub async fn create_game(
    State(pool): State<AppState>,
    Json(payload): Json<CreateGame>,
) -> Result<(StatusCode, Json<GameDetail>)> {
    let team_a_score = payload.team_a_score.unwrap_or(0);
    let team_b_score = payload.team_b_score.unwrap_or(0);

    let game_id =
        sqlx::query("INSERT INTO games (game_date, team_a_score, team_b_score, notes) VALUES (?, ?, ?, ?)")
            .bind(&payload.game_date)
            .bind(team_a_score)
            .bind(team_b_score)
            .bind(&payload.notes)
            .execute(&pool)
            .await?
            .last_insert_rowid();

    let mut game_players = Vec::new();
    for p in &payload.players {
        let score = p.score.unwrap_or(0);
        let gp_id = sqlx::query(
            "INSERT INTO game_players (game_id, player_id, player_name, team, score) VALUES (?, ?, ?, ?, ?)",
        )
        .bind(game_id)
        .bind(p.player_id)
        .bind(&p.player_name)
        .bind(&p.team)
        .bind(score)
        .execute(&pool)
        .await?
        .last_insert_rowid();

        game_players.push(GamePlayer {
            id: gp_id,
            game_id,
            player_id: p.player_id,
            player_name: p.player_name.clone(),
            team: p.team.clone(),
            score,
        });
    }

    let created_at: (String,) =
        sqlx::query_as("SELECT datetime('now', 'localtime')")
            .fetch_one(&pool)
            .await?;

    Ok((
        StatusCode::CREATED,
        Json(GameDetail {
            id: game_id,
            game_date: payload.game_date,
            team_a_score,
            team_b_score,
            notes: payload.notes,
            created_at: created_at.0,
            players: game_players,
        }),
    ))
}

pub async fn get_game(
    State(pool): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<GameDetail>> {
    let game = sqlx::query_as::<_, Game>(
        "SELECT id, game_date, team_a_score, team_b_score, notes, created_at FROM games WHERE id = ?",
    )
    .bind(id)
    .fetch_optional(&pool)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("Game {} not found", id)))?;

    let players = sqlx::query_as::<_, GamePlayer>(
        "SELECT id, game_id, player_id, player_name, team, score
         FROM game_players WHERE game_id = ? ORDER BY team, id",
    )
    .bind(id)
    .fetch_all(&pool)
    .await?;

    Ok(Json(GameDetail {
        id: game.id,
        game_date: game.game_date,
        team_a_score: game.team_a_score,
        team_b_score: game.team_b_score,
        notes: game.notes,
        created_at: game.created_at,
        players,
    }))
}

pub async fn update_game(
    State(pool): State<AppState>,
    Path(id): Path<i64>,
    Json(payload): Json<UpdateGame>,
) -> Result<Json<Game>> {
    let existing = sqlx::query_as::<_, Game>(
        "SELECT id, game_date, team_a_score, team_b_score, notes, created_at FROM games WHERE id = ?",
    )
    .bind(id)
    .fetch_optional(&pool)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("Game {} not found", id)))?;

    let team_a_score = payload.team_a_score.unwrap_or(existing.team_a_score);
    let team_b_score = payload.team_b_score.unwrap_or(existing.team_b_score);
    let notes = payload.notes.or(existing.notes.clone());

    sqlx::query("UPDATE games SET team_a_score = ?, team_b_score = ?, notes = ? WHERE id = ?")
        .bind(team_a_score)
        .bind(team_b_score)
        .bind(&notes)
        .bind(id)
        .execute(&pool)
        .await?;

    Ok(Json(Game {
        id: existing.id,
        game_date: existing.game_date,
        team_a_score,
        team_b_score,
        notes,
        created_at: existing.created_at,
    }))
}
