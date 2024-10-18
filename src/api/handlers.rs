use std::io::{Read, Write};
use std::net::TcpStream;
use crate::services::user_service;
use crate::constants::*;

pub fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    let mut request = String::new();

    match stream.read(&mut buffer) {
        Ok(size) => {
            request.push_str(String::from_utf8_lossy(&buffer[..size]).as_ref());

            let (status_line, content) = match &*request {
                r if r.starts_with("POST /users") => user_service::handle_post_request(r),
                r if r.starts_with("GET /users/") => user_service::handle_get_request(r),
                r if r.starts_with("GET /users") => user_service::handle_get_all_request(r),
                r if r.starts_with("PUT /users/") => user_service::handle_put_request(r),
                r if r.starts_with("DELETE /users/") => user_service::handle_delete_request(r),
                _ => (NOT_FOUND.to_string(), "404 Not Found".to_string()),
            };

            stream.write_all(format!("{}{}", status_line, content).as_bytes()).unwrap();
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}
