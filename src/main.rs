use std::net::TcpListener;
use makis::db::connection::set_database;
use makis::api;
fn main(){
  //Set db
    if let Err(e) = set_database(){
      println!("Error: {}", e);
      return;
    }
  //start server
  let listener = TcpListener::bind("0.0.0.0:8080").unwrap();
  println!("Server started as port 8080");

  for stream in listener.incoming() {
    match stream {
      Ok(stream) => {
          api::handlers::handle_client(stream);
      }
      Err(e)=>{
        println!("Error: {}", e)
      }
    }
  }
}

