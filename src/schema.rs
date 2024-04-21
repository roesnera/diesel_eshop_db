// @generated automatically by Diesel CLI.

diesel::table! {
    items (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        description -> Nullable<Text>,
        price -> Numeric,
        created_at -> Nullable<Timestamp>,
        quantity -> Int4,
    }
}
