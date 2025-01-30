-- This file should undo anything in `up.sql`

ALTER TABLE lesson ALTER COLUMN trainer_id DROP NOT NULL;

ALTER TABLE lesson DROP CONSTRAINT fk_trainer;

ALTER TABLE lesson DROP COLUMN trainer_id;

DROP TABLE trainer;