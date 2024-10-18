pub const DB_URL: &String = env!("DATABASE_URL");

pub fn set_database() -> Result<(), Error> {
  let mut client = Client::connect(DB_URL, NoTls)?;
  client.batch_execute("
     CREATE TABLE IF NOT EXISTS users (
            id SERIAL PRIMARY KEY,
            name VARCHAR NOT NULL,
            email VARCHAR NOT NULL
)")?;
Ok(())
}