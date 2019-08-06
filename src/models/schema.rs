table! {
    blogs (id) {
        id -> Nullable<Integer>,
        title -> Text,
        text -> Nullable<Text>,
    }
}

table! {
    posts (id) {
        id -> Nullable<Integer>,
        username -> Text,
        text -> Nullable<Text>,
    }
}

allow_tables_to_appear_in_same_query!(
    blogs,
    posts,
);
