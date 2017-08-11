CREATE TABLE members (
    id              INTEGER PRIMARY KEY,
    name            TEXT NOT NULL UNIQUE,
    team            INTEGER NOT NULL,
    last_pizza      TEXT NOT NULL
);

CREATE TABLE signins (
    id              INTEGER PRIMARY KEY,
    member_id       INTEGER NOT NULL,
    pizza           TEXT NOT NULL,
    date_in         TEXT,
    date_out        TEXT,
    FOREIGN KEY (member_id) REFERENCES members(member_id)
);