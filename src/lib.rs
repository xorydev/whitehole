use tokio::net::TcpStream;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use std::fs;

pub async fn handle_client(mut connection: TcpStream, target_dir: &String) -> std::io::Result<()> {
    let mut buffer = vec![0 as u8; 1024]; // Little bit of wiggle room.
    connection.read(&mut buffer).await?;
    let header = String::from_utf8_lossy(&buffer).to_string();
    println!("{header}");
    if !header.starts_with("GET") {
        println!("ion wanna hear it");
        return Ok(())
    }
    let client_desired_file_path = format!("{}/{}", target_dir, &header.split(" ").collect::<Vec<&str>>()[1][1..]); // Your code is not optimised if it doesn't make an inexperienced rustdev have a heart attack.
    if client_desired_file_path.contains("..") {
        println!("You should kill yourself... NOW!");
        return Ok(())
    }
    dbg!(&client_desired_file_path);
    // let file_size: usize = fs::metadata(client_desired_file_path)?.len().try_into().unwrap();
    // let mut buffer = vec![0 as u8; file_size]; // Nuke the buffer.
    let mut response: Vec<u8> = String::from("HTTP/1.1 200 OK").into_bytes();
    let mut file_contents = fs::read(&client_desired_file_path)?;
    response.append(&mut file_contents);
    connection.write(&response).await?;
    Ok(())
}
