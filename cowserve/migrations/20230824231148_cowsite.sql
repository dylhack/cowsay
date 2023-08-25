-- I would absolutely love to use an enum for the source type
-- but: https://github.com/launchbadge/sqlx/issues/1171
-- CREATE TYPE cowsay.SOURCE_TYPE AS ENUM ('file', 'url');

CREATE TABLE IF NOT EXISTS cowsay.previews (
    id TEXT PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
    cowfile_id TEXT UNIQUE NOT NULL REFERENCES cowsay.cowfiles(id),
    source_type SMALLINT NOT NULL,
    source TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    deleted_at TIMESTAMP DEFAULT NULL
);

CREATE TABLE IF NOT EXISTS cowsay.published (
    id TEXT PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
    cowfile_id TEXT UNIQUE NOT NULL REFERENCES cowsay.cowfiles(id),
    published BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    deleted_at TIMESTAMP DEFAULT NULL
);
