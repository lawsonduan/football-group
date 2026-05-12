-- Each player can now hold multiple positions, each with its own attribute ratings.
-- The old position/pac/sho/... columns on `players` are kept as dead weight so
-- existing migrations don't conflict; all live data goes into player_positions.

CREATE TABLE IF NOT EXISTS player_positions (
    id         INTEGER PRIMARY KEY AUTOINCREMENT,
    player_id  INTEGER NOT NULL,
    position   TEXT    NOT NULL,          -- fw | mf | df | gk
    pac        INTEGER NOT NULL DEFAULT 50,
    sho        INTEGER NOT NULL DEFAULT 50,
    pas        INTEGER NOT NULL DEFAULT 50,
    dri        INTEGER NOT NULL DEFAULT 50,
    def        INTEGER NOT NULL DEFAULT 50,
    phy        INTEGER NOT NULL DEFAULT 50,
    sort_order INTEGER NOT NULL DEFAULT 0, -- 0 = primary position
    FOREIGN KEY (player_id) REFERENCES players(id) ON DELETE CASCADE
);

-- Migrate any pre-existing rows so old databases keep their data.
INSERT OR IGNORE INTO player_positions
    (player_id, position, pac, sho, pas, dri, def, phy, sort_order)
SELECT id, position, pac, sho, pas, dri, def, phy, 0
FROM   players
WHERE  position NOT IN ('', 'none', 'all')
  AND  id NOT IN (SELECT DISTINCT player_id FROM player_positions);
