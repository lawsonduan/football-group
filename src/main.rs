mod error;
mod handlers;
mod models;

use axum::{
    extract::DefaultBodyLimit,
    routing::{get, put},
    Router,
};
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use std::{net::SocketAddr, str::FromStr};
use tower_http::{
    cors::CorsLayer,
    services::{ServeDir, ServeFile},
};

pub type AppState = sqlx::SqlitePool;

const PLAYERS_CSV: &str = "players.csv";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "football_group=info,tower_http=info".into()),
        )
        .init();

    let database_url =
        std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite://football.db".to_string());

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(
            SqliteConnectOptions::from_str(&database_url)?.create_if_missing(true),
        )
        .await?;

    sqlx::migrate!("./migrations").run(&pool).await?;
    seed_default_players(&pool).await?;

    tokio::fs::create_dir_all("uploads/avatars").await?;

    let static_dir = std::env::var("STATIC_DIR").unwrap_or_else(|_| "web/dist".to_string());
    let fallback = ServeFile::new(format!("{}/index.html", static_dir));
    let serve_dir = ServeDir::new(&static_dir).not_found_service(fallback);

    let app = Router::new()
        .route(
            "/api/players",
            get(handlers::players::list_players).post(handlers::players::create_player),
        )
        .route(
            "/api/players/:id",
            put(handlers::players::update_player).delete(handlers::players::delete_player),
        )
        .route(
            "/api/players/:id/avatar",
            put(handlers::uploads::upload_avatar),
        )
        .route(
            "/api/games",
            get(handlers::games::list_games).post(handlers::games::create_game),
        )
        .route(
            "/api/games/:id",
            get(handlers::games::get_game).put(handlers::games::update_game),
        )
        .route("/api/reports", get(handlers::reports::list_reports))
        .nest_service("/uploads", ServeDir::new("uploads"))
        .fallback_service(serve_dir)
        .layer(CorsLayer::permissive())
        .layer(DefaultBodyLimit::max(10 * 1024 * 1024))
        .with_state(pool);

    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8080);
    let addr = SocketAddr::from(([127, 0, 0, 1], port));

    tracing::info!("Server listening on http://{}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

/// Seeds players from `players.csv` when the players table is empty.
///
/// CSV format (header required):
///   name,position,pac,sho,pas,dri,def,phy
///
/// A player with multiple positions has one row per position; rows with the
/// same `name` are grouped together, and the first row's position is primary.
async fn seed_default_players(pool: &sqlx::SqlitePool) -> anyhow::Result<()> {
    let (count,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM players")
        .fetch_one(pool)
        .await?;

    if count > 0 {
        return Ok(());
    }

    let csv = tokio::fs::read_to_string(PLAYERS_CSV).await.map_err(|e| {
        anyhow::anyhow!(
            "Could not read '{PLAYERS_CSV}': {e}. \
             Place a players.csv file next to the binary."
        )
    })?;

    // name → player_id (insertion order preserved by processing CSV top-to-bottom)
    let mut name_to_id: std::collections::HashMap<String, i64> = std::collections::HashMap::new();
    // name → next sort_order index
    let mut sort_counters: std::collections::HashMap<String, i64> = std::collections::HashMap::new();

    let mut players_added = 0u32;
    let mut positions_added = 0u32;

    for (line_no, line) in csv.lines().enumerate() {
        if line_no == 0 || line.trim().is_empty() {
            continue;
        }
        let f: Vec<&str> = line.splitn(8, ',').collect();
        if f.len() < 8 {
            tracing::warn!(
                "players.csv line {}: expected 8 fields, got {} — skipped",
                line_no + 1,
                f.len()
            );
            continue;
        }
        let parse = |s: &str, field: &str| -> anyhow::Result<i64> {
            s.trim()
                .parse::<i64>()
                .map_err(|_| anyhow::anyhow!("line {}: invalid {field} '{s}'", line_no + 1))
        };

        let name = f[0].trim().to_string();
        let position = f[1].trim().to_string();
        let pac = parse(f[2], "pac")?;
        let sho = parse(f[3], "sho")?;
        let pas = parse(f[4], "pas")?;
        let dri = parse(f[5], "dri")?;
        let def = parse(f[6], "def")?;
        let phy = parse(f[7], "phy")?;

        // Create the player row on first encounter
        let player_id = if let Some(&id) = name_to_id.get(&name) {
            id
        } else {
            let id = sqlx::query("INSERT INTO players (name) VALUES (?)")
                .bind(&name)
                .execute(pool)
                .await?
                .last_insert_rowid();
            name_to_id.insert(name.clone(), id);
            sort_counters.insert(name.clone(), 0);
            players_added += 1;
            id
        };

        let sort_order = *sort_counters.get(&name).unwrap_or(&0);

        sqlx::query(
            "INSERT INTO player_positions \
             (player_id, position, pac, sho, pas, dri, def, phy, sort_order) \
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(player_id)
        .bind(&position)
        .bind(pac)
        .bind(sho)
        .bind(pas)
        .bind(dri)
        .bind(def)
        .bind(phy)
        .bind(sort_order)
        .execute(pool)
        .await?;

        *sort_counters.get_mut(&name).unwrap() += 1;
        positions_added += 1;
    }

    tracing::info!(
        "Seeded {players_added} players ({positions_added} position entries) from {PLAYERS_CSV}"
    );
    Ok(())
}
