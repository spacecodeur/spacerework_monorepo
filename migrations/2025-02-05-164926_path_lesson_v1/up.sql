-- Your SQL goes here

CREATE TABLE path_segment (
    id SERIAL PRIMARY KEY,
    segment_parent_id INT,
    name VARCHAR(50) NOT NULL,
    trainer_id INT
);

-- ALTER TABLE path_segment ADD CONSTRAINT trainer_id_reference FOREIGN KEY trainer_id REFERENCES trainer(id);
-- ALTER TABLE path_segment ADD CONSTRAINT segment_parent_id_reference FOREIGN KEY segment_parent_id REFERENCES path_segment(id);

-- TRUNCATE lesson; 

-- ALTER TABLE lesson DROP CONSTRAINT fk_trainer FOREIGN KEY trainer_id REFERENCES trainer(id);
-- ALTER TABLE lesson DROP COLUMN trainer_id INT;

-- ALTER TABLE lesson ADD COLUMN path_segment_id INT NOT NULL;
-- ALTER TABLE lesson ADD CONSTRAINT path_segment_id_reference FOREIGN KEY path_segment_id REFERENCES path_segment(id);



-- INSERT INTO trainer (pseudo)
-- VALUES
--     ('tester1'), -- id :3
--     ('tester2'), -- id : 4
--     ('tester3'); -- id : 5


-- INSERT INTO path_segment (segment_parent_id, name, trainer_id) VALUES 
-- -- juste un cours pour user1
-- (null, "coursjs.md", 3), -- id:1

-- -- juste un dossier pour user2
-- (null, "tata", 4),-- id:2

-- -- des dossiers et fichiers pour user3 :
-- (null, "tata", 5), -- niveau root -- id:3
-- (null, "tata2", 5),-- niveau root -- id:4
-- (null, "tata3", 5),-- niveau root -- id:5

-- (3, "toto", null), -- id:6
-- (3, "tata", null),-- id:7
-- (4, "coursjs.md", null),-- id:8
-- (5, "coursnode.md", null), -- id:9
-- (5, "coursnode2.md", null), -- id:10
-- (7, "coursphp.md", null), -- id:11
-- (7, "tata", null); -- id:12


-- INSERT INTO lesson (path_segment_id, content)
-- VALUES
-- (1, '#coursjs\n
-- ça va ?\n
-- {{{widget-inline "param1" "**param2**" " **param3** "}}}\n
-- \{{{widget-inline "param1" "param2" "param3"\n
-- {{{widget-multiline "param1" "param2" "param3"}}}\n
-- content1\n
-- <h2>titre deja présent</h2>\n
-- <ul>\n
-- <li>liste<li>\n
-- </ul>\n
-- <div># coucou ! **test** </div>\n
-- \n
-- content2\n
-- \n
-- content3\n
-- widget-multilie}}}'),
-- (8, '#coursjs\n ! \n blablabla'),
-- (9, '#coursnode\n ! \n blablabla'),
-- (10, '#coursnode2\n ! \n blablabla'),
-- (11, '');