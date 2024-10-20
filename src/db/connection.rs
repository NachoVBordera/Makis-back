use postgres::{Client, Error, NoTls};
use std::env;
use dotenv::dotenv;

pub fn get_db_url() -> String {
  let db_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL environment variable must be set");
  return db_url;
}

pub fn set_database() -> Result<(), Error> {
    dotenv().ok(); 
  
    let db_url = get_db_url();

    let mut client = Client::connect(&db_url, NoTls)?;
    
    client.batch_execute("
        CREATE TABLE IF NOT EXISTS users (
            id SERIAL PRIMARY KEY,
            name VARCHAR NOT NULL,
            email VARCHAR NOT NULL
        )
    ")?;
    
    Ok(())
}