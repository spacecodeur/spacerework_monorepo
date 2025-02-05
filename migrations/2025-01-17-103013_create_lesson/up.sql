-- Your SQL goes here

CREATE TABLE lesson (
  id SERIAL PRIMARY KEY,
  content TEXT NOT NULL
);

INSERT INTO lesson(content)
VALUES
('# coucou\n
ça va ?\n
{{{widget-inline "param1" "**param2**" " **param3** "}}}\n
\{{{widget-inline "param1" "param2" "param3"\n
{{{widget-multiline "param1" "param2" "param3"}}}\n
content1\n
<h2>titre deja présent</h2>\n
<ul>\n
<li>liste<li>\n
</ul>\n
<div># coucou ! **test** </div>\n
\n
content2\n
\n
content3\n
widget-multilie}}}'),
('# un autre cours ! \n blablabla');