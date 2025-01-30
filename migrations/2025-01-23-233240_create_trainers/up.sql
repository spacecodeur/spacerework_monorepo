-- Your SQL goes here

CREATE TABLE trainer (
    id SERIAL PRIMARY KEY,
    pseudo VARCHAR(50) UNIQUE NOT NULL
);

INSERT INTO trainer (pseudo)
VALUES
    ('spacecodeur'),
    ('hazefury');

ALTER TABLE lesson ADD COLUMN trainer_id INT;

UPDATE lesson SET trainer_id = (SELECT id FROM trainer LIMIT 1);

ALTER TABLE lesson
ADD CONSTRAINT fk_trainer FOREIGN KEY (trainer_id) REFERENCES trainer (id);

ALTER TABLE lesson ALTER COLUMN trainer_id SET NOT NULL;