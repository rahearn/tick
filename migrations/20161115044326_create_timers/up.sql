CREATE TABLE timers (
  id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  name VARCHAR NOT NULL,
  start_time INTEGER NOT NULL,
  end_time INTEGER NOT NULL DEFAULT 0,
  start_entry TEXT NOT NULL DEFAULT '',
  end_entry TEXT NOT NULL DEFAULT '',
  running INTEGER NOT NULL DEFAULT 0
)
