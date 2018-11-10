#![allow(proc_macro_derive_resolution_fallback)]

use uuid::Uuid;

#[derive(Queryable, Serialize, Deserialize)]
pub struct Hero {
    pub id: Uuid,
    pub name: String,
    pub age: i32,
}
