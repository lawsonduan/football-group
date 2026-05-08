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

const DEFAULT_PLAYERS: &[(&str, &str)] = &[
    ("C罗", "fw"),
    ("梅西", "fw"),
    ("姆巴佩", "fw"),
    ("哈兰德", "fw"),
    ("莱万", "fw"),
    ("凯恩", "fw"),
    ("内马尔", "fw"),
    ("德布劳内", "mf"),
    ("莫德里奇", "mf"),
    ("克罗斯", "mf"),
    ("B席", "mf"),
    ("厄德高", "mf"),
    ("维拉蒂", "mf"),
    ("罗德里", "mf"),
    ("赖斯", "mf"),
    ("范戴克", "df"),
    ("迪亚斯", "df"),
    ("萨利巴", "df"),
    ("格瓦迪奥尔", "df"),
    ("阿劳霍", "df"),
    ("德里赫特", "df"),
    ("坎塞洛", "df"),
    ("卡马文加", "all"),
    ("楚阿梅尼", "all"),
    ("贝林厄姆", "all"),
    ("福登", "all"),
    ("萨卡", "all"),
    ("拉什福德", "fw"),
    ("格拉利什", "mf"),
    ("马赫雷斯", "fw"),
    ("阿尔瓦雷斯", "fw"),
    ("罗德里戈", "fw"),
    ("维尼修斯", "fw"),
    ("诺伊尔", "gk"),
    ("库尔图瓦", "gk"),
    ("阿利森", "gk"),
];

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

    // Ensure avatar upload directory exists
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
        .nest_service("/uploads", ServeDir::new("uploads"))
        .fallback_service(serve_dir)
        .layer(CorsLayer::permissive())
        .layer(DefaultBodyLimit::max(10 * 1024 * 1024)) // 10 MB global limit
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

/// Returns (pac, sho, pas, dri, def, phy) defaults by position.
fn default_stats(position: &str) -> (i64, i64, i64, i64, i64, i64) {
    match position {
        "fw"  => (78, 80, 72, 78, 35, 65),
        "mf"  => (72, 65, 80, 75, 60, 70),
        "df"  => (70, 45, 65, 60, 82, 78),
        "gk"  => (60, 15, 65, 45, 83, 75),
        "all" => (73, 70, 73, 78, 57, 68),
        _     => (50, 50, 50, 50, 50, 50),
    }
}

async fn seed_default_players(pool: &sqlx::SqlitePool) -> anyhow::Result<()> {
    let (count,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM players")
        .fetch_one(pool)
        .await?;

    if count > 0 {
        return Ok(());
    }

    tracing::info!("Seeding {} default players", DEFAULT_PLAYERS.len());
    for (name, position) in DEFAULT_PLAYERS {
        let (pac, sho, pas, dri, def, phy) = default_stats(position);
        sqlx::query(
            "INSERT INTO players (name, position, pac, sho, pas, dri, def, phy) \
             VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(name)
        .bind(position)
        .bind(pac)
        .bind(sho)
        .bind(pas)
        .bind(dri)
        .bind(def)
        .bind(phy)
        .execute(pool)
        .await?;
    }

    Ok(())
}
