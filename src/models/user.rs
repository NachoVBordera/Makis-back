//Model: User struct with id, name, email

#[derive(Serialize, Deserialize)]
pub struct User {
  id: Option<i32>,
  name: String,
  email: String,
}