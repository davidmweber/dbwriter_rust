// @generated automatically by Diesel CLI.

diesel::table! {
    samples (id) {
        id -> Int8,
        name -> Varchar,
        timestamp -> Timestamptz,
        v0 -> Nullable<Float4>,
        v1 -> Nullable<Float4>,
    }
}
