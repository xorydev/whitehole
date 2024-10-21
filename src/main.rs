use std::net;
use std::path::Path;
use whitehole::handle_client;

fn main() -> std::io::Result<()> {
  let args = &std::env::args().collect::<Vec<String>>()[1..];
  let target_dir = Path::new(&args[0]);
  if !target_dir.is_dir() {
    panic!("Target is not a directory");
  }

  // Alphanumerical for "WHIH" or "Whitehole". No idea why I did this.
  let listener = net::TcpListener::bind("0.0.0.0:23898")?;

  for connection in listener.incoming() {
    handle_client(connection?, &args[0])?; // See lib.rs for handle_client()
  }

  Ok(())
}
