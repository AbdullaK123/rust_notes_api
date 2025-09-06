// @generated automatically by Diesel CLI.

diesel::table! {
    notes (id) {
        id -> Uuid,
        user_id -> Uuid,
        title -> Text,
        content -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        username -> Text,
        email -> Text,
        password_hash -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::joinable!(notes -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    notes,
    users,
);
