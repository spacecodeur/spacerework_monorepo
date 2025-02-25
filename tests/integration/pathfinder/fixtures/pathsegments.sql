INSERT INTO public.path_segment(id, name, segment_parent_id, trainer_id)
VALUES
    (1, "tata", null, 1),
    (2, "tata2", null, 1),
    (3, "tata3", null, 1),
    (4, "toto", 1, 1),
    (5, "tata", 1, null),
    (6, "coursjs.md", 2, null),
    (7, "coursnode.md", 3, null),
    (8, "coursnode2.md", 3, null),
    (9, "coursphp.md", 5, null),
    (10, "tata", 5, null);