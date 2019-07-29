table! {
    events (id) {
        id -> Int4,
        content -> Text,
        finished -> Bool,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}
