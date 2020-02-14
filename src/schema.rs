table! {
    items (id) {
        id -> Nullable<Uuid>,
        name -> Varchar,
        buy_time -> Nullable<Timestamp>,
        owner -> Nullable<Varchar>,
        create_at -> Nullable<Timestamp>,
        type_id -> Nullable<Int2>,
        comment -> Nullable<Varchar>,
    }
}

table! {
    types (id) {
        id -> Nullable<Int2>,
        name -> Varchar,
        create_at -> Nullable<Timestamp>,
        comment -> Nullable<Varchar>,
    }
}

joinable!(items -> types (type_id));

allow_tables_to_appear_in_same_query!(
    items,
    types,
);
