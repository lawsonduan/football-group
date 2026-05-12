use serde::{Deserialize, Serialize};

/// A single position record for a player (e.g. FW or MF), with its own attribute ratings.
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, Clone)]
pub struct PlayerPosition {
    pub id: i64,
    pub player_id: i64,
    pub position: String,
    pub pac: i64,
    pub sho: i64,
    pub pas: i64,
    pub dri: i64,
    pub def: i64,
    pub phy: i64,
    pub sort_order: i64,
}

/// A player together with all of their position records.
/// The first element of `positions` is the primary position.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Player {
    pub id: i64,
    pub name: String,
    pub avatar: Option<String>,
    pub positions: Vec<PlayerPosition>,
}

/// Used internally to query the bare `players` row (no positions).
#[derive(Debug, sqlx::FromRow, Clone)]
pub struct PlayerRow {
    pub id: i64,
    pub name: String,
    pub avatar: Option<String>,
}

/// One position entry inside a create/update request body.
#[derive(Debug, Deserialize, Clone)]
pub struct PositionInput {
    pub position: String,
    pub pac: Option<i64>,
    pub sho: Option<i64>,
    pub pas: Option<i64>,
    pub dri: Option<i64>,
    pub def: Option<i64>,
    pub phy: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct CreatePlayer {
    pub name: String,
    /// At least one position is required; the first is treated as primary.
    pub positions: Vec<PositionInput>,
}

#[derive(Debug, Deserialize)]
pub struct UpdatePlayer {
    pub name: Option<String>,
    /// When provided, replaces all existing position records.
    pub positions: Option<Vec<PositionInput>>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Game {
    pub id: i64,
    pub game_date: String,
    pub team_a_score: i64,
    pub team_b_score: i64,
    pub notes: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct GamePlayer {
    pub id: i64,
    pub game_id: i64,
    pub player_id: Option<i64>,
    pub player_name: String,
    pub team: String,
    pub score: i64,
}

#[derive(Debug, Deserialize)]
pub struct GamePlayerInput {
    pub player_id: Option<i64>,
    pub player_name: String,
    pub team: String,
    pub score: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct CreateGame {
    pub game_date: String,
    pub team_a_score: Option<i64>,
    pub team_b_score: Option<i64>,
    pub notes: Option<String>,
    pub players: Vec<GamePlayerInput>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateGame {
    pub team_a_score: Option<i64>,
    pub team_b_score: Option<i64>,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct GameDetail {
    pub id: i64,
    pub game_date: String,
    pub team_a_score: i64,
    pub team_b_score: i64,
    pub notes: Option<String>,
    pub created_at: String,
    pub players: Vec<GamePlayer>,
}
