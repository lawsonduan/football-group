use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, Clone)]
pub struct Player {
    pub id: i64,
    pub name: String,
    pub position: String,
    pub avatar: Option<String>,
    pub pac: i64,
    pub sho: i64,
    pub pas: i64,
    pub dri: i64,
    pub def: i64,
    pub phy: i64,
}

#[derive(Debug, Deserialize)]
pub struct CreatePlayer {
    pub name: String,
    pub position: Option<String>,
    pub pac: Option<i64>,
    pub sho: Option<i64>,
    pub pas: Option<i64>,
    pub dri: Option<i64>,
    pub def: Option<i64>,
    pub phy: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct UpdatePlayer {
    pub name: Option<String>,
    pub position: Option<String>,
    pub pac: Option<i64>,
    pub sho: Option<i64>,
    pub pas: Option<i64>,
    pub dri: Option<i64>,
    pub def: Option<i64>,
    pub phy: Option<i64>,
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
