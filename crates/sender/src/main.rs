use std::{
    io::{self, Error, Result, Write},
    os::unix::net::UnixStream,
};

use protocol::Request;

fn send_message(content: String) -> Result<()> {
    let mut stream = UnixStream::connect("/tmp/test-icp.sock")?;

    let serialized = bincode::serialize(&Request::Message(content))
        .map_err(|e| Error::new(io::ErrorKind::Other, e))?;

    let len = serialized.len() as u32;

    stream.write_all(&len.to_be_bytes())?;

    stream.write_all(&serialized)?;

    Ok(())
}

fn main() -> Result<()> {
    let mut input = String::new();

    loop {
        print!("> ");
        io::stdout().flush()?;

        input.clear();
        io::stdin().read_line(&mut input)?;

        let trimmed = input.trim();

        if trimmed == "exit" {
            println!("Fermeture du client.");
            break;
        }

        if !trimmed.is_empty() {
            match send_message(trimmed.to_string()) {
                Ok(()) => println!("  [OK] Envoyé"),
                Err(e) => eprintln!("  [Erreur] Impossible d'envoyer : {}", e),
            }
        }
    }

    Ok(())
}
