// @generated automatically by Diesel CLI.

diesel::table! {
    lesson (id) {
        id -> Int4,
        content -> Text,
        trainer_id -> Int4,
    }
}

diesel::table! {
    trainer (id) {
        id -> Int4,
        #[max_length = 50]
        pseudo -> Varchar,
    }
}

diesel::joinable!(lesson -> trainer (trainer_id));

diesel::allow_tables_to_appear_in_same_query!(
    lesson,
    trainer,
);
