use std::fs;
use std::io::{Read, Write};
use std::net::{Shutdown, TcpStream};

pub fn handle_client(mut connection: TcpStream, target_dir: &String) -> std::io::Result<()> {
  let mut buffer: Vec<u8> = vec![];
  match connection.read(&mut buffer) {
    Ok(0) => {
      println!("nvm");
      return Ok(());
    }
    Err(e) => {
      return Err(e);
    }
    Ok(_) => {}
  }

  let req = String::from_utf8_lossy(&buffer).to_string();
  println!("{req}");

  if !req.starts_with("GET") {
    connection.shutdown(Shutdown::Both)?;
    println!("ion wanna hear it");
    return Ok(());
  }

  let client_desired_file_path = format!(
    "{}/{}",
    target_dir,
    &req.split(" ").collect::<Vec<&str>>()[1][1..]
  ); // Your code is not optimised if it doesn't make an inexperienced rustdev have a heart attack.

  if client_desired_file_path.contains("..") {
    connection.shutdown(Shutdown::Both)?;
    println!("You should kill yourself... NOW!");
    return Ok(());
  }

  dbg!(&client_desired_file_path);
  let mut file_contents = fs::read(&client_desired_file_path)?;
  let content_length = file_contents.len();
  let mut response: Vec<u8> =
    format!("HTTP/1.1 200 OK\r\nContent-Length: {content_length}\r\n\r\n").into_bytes();
  response.append(&mut file_contents);

  let written = connection.write(&response)?;
  let response_len = response.len();
  if written != response.len() {
    eprintln!("written: {written}, actual response length: {response_len}");
  }

  Ok(())
}
