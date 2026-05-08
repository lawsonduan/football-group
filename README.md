# 足球比赛分组系统

A football team randomizer with a Rust/SQLite backend for recording game history.

## Architecture

```
football-group/
├── Cargo.toml                  # Rust backend
├── migrations/
│   └── 001_initial.sql
├── src/                        # Rust source
│   ├── main.rs
│   ├── models.rs
│   ├── error.rs
│   └── handlers/
│       ├── players.rs
│       └── games.rs
├── web/                        # Vue 3 + TypeScript frontend
│   ├── package.json
│   ├── vite.config.ts
│   ├── src/
│   │   ├── App.vue
│   │   ├── api/index.ts
│   │   ├── stores/             # Pinia stores
│   │   ├── components/         # Vue components
│   │   └── assets/main.css
│   └── dist/                   # production build output (git-ignored)
└── football.db                 # SQLite database (auto-created on first run)
```

## Requirements

- [Rust](https://rustup.rs/) 1.70+

## Running

### Production (single server)

```bash
# 1. Build the Vue frontend
cd web && npm install && npm run build && cd ..

# 2. Start the Rust server (serves API + web/dist/)
cargo run
```

Open **http://localhost:8080** in your browser.

### Development (hot-reload)

```bash
# Terminal 1 — Rust API on :8080
cargo run

# Terminal 2 — Vite dev server on :5173 (proxies /api → :8080)
cd web && npm run dev
```

Open **http://localhost:5173** for hot-reloading during development.

The SQLite database (`football.db`) is created automatically on first run.

## Environment Variables

| Variable       | Default                  | Description                        |
|----------------|--------------------------|------------------------------------|
| `DATABASE_URL` | `sqlite://football.db`   | SQLite database path               |
| `PORT`         | `8080`                   | HTTP server port                   |
| `STATIC_DIR`   | `..`                     | Directory containing `index.html`  |
| `RUST_LOG`     | `football_backend=info`  | Log level                          |

## API Endpoints

### Players

| Method   | Path                  | Description            |
|----------|-----------------------|------------------------|
| `GET`    | `/api/players`        | List all players       |
| `POST`   | `/api/players`        | Add a player           |
| `PUT`    | `/api/players/:id`    | Update a player        |
| `DELETE` | `/api/players/:id`    | Delete a player        |

### Games

| Method | Path            | Description                   |
|--------|-----------------|-------------------------------|
| `GET`  | `/api/games`    | List all games                |
| `POST` | `/api/games`    | Record a new game             |
| `GET`  | `/api/games/:id`| Get game details with players |
| `PUT`  | `/api/games/:id`| Update game score / notes     |

### Example: record a game

```bash
curl -X POST http://localhost:8080/api/games \
  -H 'Content-Type: application/json' \
  -d '{
    "game_date": "2026-05-08",
    "team_a_score": 3,
    "team_b_score": 2,
    "notes": "第1场",
    "players": [
      {"player_id": 1, "player_name": "C罗", "team": "A", "score": 2},
      {"player_id": 2, "player_name": "梅西", "team": "B", "score": 1}
    ]
  }'
```

## Database Schema

```sql
players      (id, name, position, created_at)
games        (id, game_date, team_a_score, team_b_score, notes, created_at)
game_players (id, game_id, player_id, player_name, team, score)
```

`game_players.player_id` is nullable — historical records are preserved even if a player is deleted from the roster.
