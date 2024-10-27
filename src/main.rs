use tokio::net;
use std::path::PathBuf;
use std::sync::Arc;
use whitehole::handle_client;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let target_dir = PathBuf::from(
        std::env::args()
            .nth(1)
            .expect("No target directory provided"),
    );
    if !target_dir.is_dir() {
        panic!("Target is not a directory");
    }

    let listener = Arc::new(net::TcpListener::bind("0.0.0.0:23898").await?); // Alphanumerical for "WHIH" or
                                                             // "Whitehole". No idea why I did
                                                             // this.
    for _ in 0..num_cpus::get() {
        let target_dir = target_dir.clone();
        let listener = listener.clone();

        tokio::spawn(async move {
            loop {
                let stream = match listener.accept().await {
                    Err(e) => {
                        eprintln!("{e}");
                        continue;
                    },
                    Ok((stream, _)) => stream,
                };

                if let Err(e) = handle_client(stream, &target_dir.to_str().unwrap().to_string()).await {
                    eprintln!("{e}");
                }
            }
        });
    }

    Ok(())
}
