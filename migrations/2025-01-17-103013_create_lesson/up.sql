-- Your SQL goes here

CREATE TABLE lesson (
  id SERIAL PRIMARY KEY,
  content TEXT NOT NULL
);

INSERT INTO lesson(content)
VALUES
('cours sur nodejs \n Bonjour !'), 
('un autre cours ! \n blablabla');