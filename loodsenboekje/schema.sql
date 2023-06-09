DROP TABLE IF EXISTS user;
DROP TABLE IF EXISTS entry;
DROP TABLE IF EXISTS entry_user;

CREATE TABLE user (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT UNIQUE NOT NULL
);

CREATE TABLE entry_user (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    entry_id INTEGER NOT NULL,
    FOREIGN KEY (user_id) REFERENCES user (id),
    FOREIGN KEY (entry_id) REFERENCES entry (id)
);

CREATE TABLE entry (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    how TEXT UNIQUE NOT NULL,
    created TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
