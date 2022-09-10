// @generated automatically by Diesel CLI.

diesel::table! {
    list_of_drivers (phone) {
        phone -> Int4,
        name -> Nullable<Text>,
        email -> Nullable<Text>,
        password -> Text,
        taxi_type -> Text,
    }
}

diesel::table! {
    list_of_users (phone) {
        phone -> Int4,
        name -> Nullable<Text>,
        email -> Nullable<Text>,
        password -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    list_of_drivers,
    list_of_users,
);
