CREATE TABLE signins2 (
    id              INTEGER PRIMARY KEY,
    member_id       INTEGER NOT NULL,
    pizza           TEXT NOT NULL,
    date_in         TEXT,
    date_out        TEXT,
    FOREIGN KEY (member_id) REFERENCES members(id) ON DELETE CASCADE
);

INSERT INTO signins2 (id, member_id, pizza, date_in, date_out)
   SELECT id, member_id, pizza, date_in, date_out FROM SIGNINS;
DROP TABLE signins;
ALTER TABLE signins2 RENAME TO signins;
