// @generated automatically by Diesel CLI.

diesel::table! {
    lesson (id) {
        id -> Int4,
        content -> Text,
        path_segment_id -> Int4,
    }
}

diesel::table! {
    path_segment (id) {
        id -> Int4,
        segment_parent_id -> Nullable<Int4>,
        #[max_length = 50]
        name -> Varchar,
        trainer_id -> Nullable<Int4>,
    }
}

diesel::table! {
    trainer (id) {
        id -> Int4,
        #[max_length = 50]
        pseudo -> Varchar,
    }
}

diesel::joinable!(lesson -> path_segment (path_segment_id));
diesel::joinable!(path_segment -> trainer (trainer_id));

diesel::allow_tables_to_appear_in_same_query!(
    lesson,
    path_segment,
    trainer,
);
