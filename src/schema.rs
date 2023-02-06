// @generated automatically by Diesel CLI.

diesel::table! {
    danmakus (id) {
        id -> Int4,
        room_id -> Int8,
        user_id -> Int8,
        user_name -> Text,
        message -> Text,
        essential_message -> Text,
        is_emoticon -> Bool,
        flag -> Int4,
        medal -> Nullable<Text>,
        timestamp -> Timestamptz,
    }
}

diesel::table! {
    enters (id) {
        id -> Int4,
        room_id -> Int8,
        user_id -> Int8,
        user_name -> Text,
        medal -> Nullable<Text>,
        timestamp -> Timestamptz,
    }
}

diesel::table! {
    gifts (id) {
        id -> Int4,
        room_id -> Int8,
        user_id -> Int8,
        user_name -> Text,
        gift_id -> Int4,
        gift_name -> Text,
        gift_price -> Int4,
        gift_count -> Int4,
        gift_paid -> Bool,
        medal -> Nullable<Text>,
        timestamp -> Timestamptz,
    }
}

diesel::table! {
    guard_buys (id) {
        id -> Int4,
        room_id -> Int8,
        user_id -> Int8,
        user_name -> Text,
        guards_level -> Int2,
        price -> Int4,
        timestamp -> Timestamptz,
    }
}

diesel::table! {
    popularitys (id) {
        id -> Int4,
        count -> Int4,
        room_id -> Int8,
        timestamp -> Timestamptz,
    }
}

diesel::table! {
    superchats (id) {
        id -> Int4,
        room_id -> Int8,
        user_id -> Int8,
        user_name -> Text,
        price -> Int4,
        message -> Text,
        medal -> Nullable<Text>,
        timestamp -> Timestamptz,
    }
}

diesel::table! {
    watcheds (id) {
        id -> Int4,
        count -> Int4,
        room_id -> Int8,
        timestamp -> Timestamptz,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    danmakus,
    enters,
    gifts,
    guard_buys,
    popularitys,
    superchats,
    watcheds,
);
