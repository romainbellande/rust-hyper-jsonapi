extern crate dotenv;
extern crate envy;

use serde_derive;

#[derive(Deserialize, Debug)]
pub struct Config {
  postgres_user: String,
  postgres_password: String,
  postgres_db: String,
  postgres_port: String
}

pub fn load() {
    dotenv::dotenv().ok();
    match envy::from_env::<Config>() {
      Ok(config) => println!("{:#?}", config),
      Err(error) => panic!("{:#?}", error)
   }
}

pub fn get() -> Config {
  return envy::from_env::<Config>().unwrap();
}

pub fn get_database_url() -> String {
  return "postgres://".to_owned() +
    &get().postgres_user +
    &":".to_owned() +
    &get().postgres_password +
    &"@localhost:".to_owned() +
    &get().postgres_port +
    &"/".to_owned() +
    &get().postgres_db;
}
