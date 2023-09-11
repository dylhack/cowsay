-- Add migration script here
UPDATE cowsay.previews SET source = CONCAT('file://', source) WHERE source_type = 0;
ALTER TABLE cowsay.previews DROP COLUMN source_type;
