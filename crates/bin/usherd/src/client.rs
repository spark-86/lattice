use anyhow::{Result, anyhow};
use base64::{Engine as _, engine};
use futures::{SinkExt, StreamExt};
use lattice::{
    Rhex,
    usher::{self, location::UsherLocation},
};
use std::path::PathBuf;

pub async fn run(usher: String, rhex_file: PathBuf, usher_map: PathBuf) -> Result<()> {
    // Prep everything
    let usher_key: [u8; 32] = engine::general_purpose::URL_SAFE_NO_PAD
        .decode(usher)
        .unwrap()
        .try_into()
        .unwrap();
    let usher_map: usher::UsherMap = usher::map::disk_from(&usher_map.to_str().unwrap());
    let rhex = std::fs::read(rhex_file)?;
    let rhex: Vec<Rhex> = minicbor::decode(&rhex)?;

    // Lookup
    let usher = usher_map.get(&usher_key);
    if usher.is_none() {
        return Err(anyhow!("Usher not found"));
    }
    let usher = usher.unwrap();

    // Conncet and send based on location
    match &usher.location {
        // Using good ol' internet. Standard TCP connection.
        UsherLocation::Internet { .. } | UsherLocation::Local { .. } => {
            let usher_addr = match &usher.location {
                UsherLocation::Internet { ip, port } => format!("{}:{}", ip, port),
                UsherLocation::Local { port } => format!("127.0.0.1:{}", port),
                _ => unreachable!(),
            };

            let stream = tokio::net::TcpStream::connect(usher_addr).await?;
            let mut framed = tokio_util::codec::Framed::new(
                stream,
                tokio_util::codec::LengthDelimitedCodec::new(),
            );

            for rhex in rhex {
                let mut buf = Vec::new();
                minicbor::encode(&rhex, &mut buf)?;
                framed.send(buf.clone().into()).await?;
            }

            while let Some(response_result) = framed.next().await {
                let response = response_result?;
                let response: Vec<Rhex> = minicbor::decode(&response)?;
                println!("Received {} items", response.len());
            }
        }
        UsherLocation::Tor { .. } | UsherLocation::Unknown => {
            // We'll get to Tor one day, I promise lol
            unimplemented!()
        }
    }

    Ok(())
}
