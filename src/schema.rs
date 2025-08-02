// @generated automatically by Diesel CLI.

diesel::table! {
    images (id) {
        id -> Int4,
        #[max_length = 255]
        url -> Varchar,
        created_at -> Nullable<Timestamp>,
    }
}

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

diesel::table! {
    items_images (id) {
        id -> Int4,
        item_id -> Int4,
        image_id -> Int4,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    roles (id) {
        id -> Int4,
        #[max_length = 64]
        code -> Varchar,
        #[max_length = 128]
        name -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 64]
        username -> Varchar,
        #[max_length = 128]
        password -> Varchar,
        #[max_length = 128]
        email -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::table! {
    users_roles (id) {
        id -> Int4,
        user_id -> Int4,
        role_id -> Int4,
    }
}

diesel::joinable!(items_images -> images (image_id));
diesel::joinable!(items_images -> items (item_id));
diesel::joinable!(users_roles -> roles (role_id));
diesel::joinable!(users_roles -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    images,
    items,
    items_images,
    roles,
    users,
    users_roles,
);
