table! {
    comments (vid) {
        vid -> Integer,
        id -> Integer,
        product_id -> Integer,
        title -> Text,
        body -> Text,
        rating -> Integer,
    }
}

table! {
    products (id) {
        id -> Integer,
        title_fa -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    comments,
    products,
);
