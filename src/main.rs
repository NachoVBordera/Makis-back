use std::net::TcpListener;


fn main(){
  //Set db
    if let Err(e) = set_database(){
      println!("Error: {}", e);
      return;
    }
  //start server
  let listener = TcpListener::bind(format!(0.0.0.0:8080)).unwrap();
  println!("Server started as port 8080");

  for stream in listener.incoming() {
    match stream {
      ok(stream) => {
          api::handlers::handle_client(stream);
      }
      Err(e)=>{
        println!("Error: {}", e)
      }
    }
  }
}

