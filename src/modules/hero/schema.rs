#![allow(proc_macro_derive_resolution_fallback)]

table! {
    heroes (id) {
        id -> Uuid,
        name -> Varchar,
        age -> Int4,
    }
}
