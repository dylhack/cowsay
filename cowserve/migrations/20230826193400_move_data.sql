CREATE TABLE IF NOT EXISTS cowsay.cowdata (
    id TEXT PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
    data TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    deleted_at TIMESTAMP DEFAULT NULL
);

ALTER TABLE cowsay.cowfiles ADD COLUMN data_id TEXT REFERENCES cowsay.cowdata(id);
INSERT INTO cowsay.cowdata (data) SELECT data FROM cowsay.cowfiles;
UPDATE cowsay.cowfiles SET data_id = cowsay.cowdata.id FROM cowsay.cowdata WHERE cowsay.cowdata.data = cowsay.cowfiles.data;
ALTER TABLE cowsay.cowfiles ALTER COLUMN data_id SET NOT NULL;
ALTER TABLE cowsay.cowfiles DROP COLUMN data;
