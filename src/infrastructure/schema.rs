// @generated automatically by Diesel CLI.

diesel::table! {
    admins (id) {
        id -> Integer,
        username -> Text,
        password_hash -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    bookings (id) {
        id -> Integer,
        room_id -> Integer,
        user_id -> Integer,
        start_time -> Timestamp,
        end_time -> Timestamp,
        status -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    rooms (id) {
        id -> Integer,
        name -> Text,
        status -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        username -> Text,
        password_hash -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(bookings -> rooms (room_id));
diesel::joinable!(bookings -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    admins,
    bookings,
    rooms,
    users,
);
