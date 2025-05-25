// @generated automatically by Diesel CLI.

diesel::table! {
    bookings (id) {
        id -> Integer,
        user_id -> Integer,
        room_id -> Integer,
        start_time -> Timestamp,
        end_time -> Timestamp,
    }
}

diesel::table! {
    rooms (id) {
        id -> Integer,
        name -> Text,
        status -> Text,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        username -> Text,
        password_hash -> Text,
        role -> Text,
    }
}

diesel::joinable!(bookings -> rooms (room_id));
diesel::joinable!(bookings -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    bookings,
    rooms,
    users,
);
