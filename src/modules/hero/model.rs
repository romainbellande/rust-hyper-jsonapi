#![allow(proc_macro_derive_resolution_fallback)]

use uuid::Uuid;
use diesel;
use diesel::prelude::*;
use japi::model::JApiModel;
use japi::japi_model;
use super::schema::heroes;

#[table_name = "heroes"]
#[derive(Serialize, Insertable, Deserialize, Debug)]
pub struct HeroDto {
    pub name: String,
    pub age: i32,
}

#[table_name = "heroes"]
#[derive(Serialize, Deserialize, Queryable, Insertable, AsChangeset)]
pub struct Hero {
    pub id: Uuid,
    pub name: String,
    pub age: i32,
}
japi_model!(Hero; "hero");

impl Hero {
  pub fn read(connection: &PgConnection) -> Vec<Hero> {
    heroes::table.order(heroes::id).load::<Hero>(connection).unwrap()
  }

  pub fn create(hero: HeroDto, connection: &PgConnection) -> Hero {
    diesel::insert_into(heroes::table)
      .values(hero)
      .execute(connection)
      .expect("Error creating new hero");

    heroes::table.order(heroes::id.desc()).first(connection).unwrap()
  }
}
