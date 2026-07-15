use protocol::Request;
use tokio::io::AsyncReadExt;
use tokio::net::UnixListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let socket_path = "/tmp/test-icp.sock";
    let _ = std::fs::remove_file(socket_path);

    let listener = UnixListener::bind(socket_path)?;

    loop {
        let (mut stream, _) = listener.accept().await?;

        tokio::spawn(async move {
            loop {
                let msg_len = match stream.read_u32().await {
                    Ok(len) => len as usize,
                    Err(_) => break,
                };

                let mut buf = vec![0u8; msg_len];

                if stream.read_exact(&mut buf).await.is_err() {
                    break;
                }

                if let Ok(request) = bincode::deserialize::<Request>(&buf) {
                    println!("Requête reçue : {:?}", request);
                }
            }
        });
    }
}
