// @generated automatically by Diesel CLI.

diesel::table! {
    devices (id) {
        id -> Text,
        name -> Text,
        room_id -> Text,
    }
}

diesel::table! {
    rooms (id) {
        id -> Text,
        name -> Text,
    }
}

diesel::joinable!(devices -> rooms (room_id));

diesel::allow_tables_to_appear_in_same_query!(devices, rooms,);
