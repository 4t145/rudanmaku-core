// @generated automatically by Diesel CLI.

diesel::table! {
    danmakus (id) {
        id -> Int4,
        room_id -> Int8,
        user_id -> Int8,
        user_name -> Text,
        message -> Text,
        essential_message -> Text,
        flag -> Int4,
        medal -> Nullable<Text>,
        timestamp -> Timestamptz,
    }
}
