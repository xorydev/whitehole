use tokio::net;
use std::path::Path;
use whitehole::handle_client;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let args = &std::env::args().collect::<Vec<String>>()[1..];
    let target_dir = Path::new(&args[0]);
    if !target_dir.is_dir() {
        panic!("Target is not a directory");
    }

    let listener = net::TcpListener::bind("0.0.0.0:23898").await?; // Alphanumerical for "WHIH" or
                                                             // "Whitehole". No idea why I did
                                                             // this.
    loop { 
        let (socket, _) = listener.accept().await?;
        handle_client(socket, &target_dir.to_str().unwrap().to_string()).await?;
    }
}
