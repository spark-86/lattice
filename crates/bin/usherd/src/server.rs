use anyhow::Result;
use futures::{SinkExt, StreamExt};
use lattice::Rhex;

use crate::rebuild;

pub async fn run(port: u16, rebuild: bool) -> Result<()> {
    let addr = format!("0.0.0.0:{}", port);

    // If rebuild=true we fire off the rebuilt bootstrap procedure,
    // otherwise we build from our existing cache

    let lattice = if rebuild {
        rebuild::rebuild("./".to_string()).unwrap()
    } else {
        let mut building_lattice = lattice::Lattice::new();
        let status = building_lattice.startup("./".to_string());
        status.unwrap();
        building_lattice
    };
    println!("🧬 Lattice is live! {} scopes loaded", lattice.scopes.len());

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    println!("🟢 Server listening on {}", addr);

    loop {
        let (stream, _) = listener.accept().await?;

        tokio::spawn(async move {
            if let Err(e) = handle_connection(stream).await {
                eprintln!("Connection error: {:?}", e);
            }
        });
    }
}

async fn handle_connection(stream: tokio::net::TcpStream) -> Result<()> {
    // We use LengthDelimitedCodec so we don't have to worry about
    // TCP fragmenting our CBOR blobs.
    let mut framed =
        tokio_util::codec::Framed::new(stream, tokio_util::codec::LengthDelimitedCodec::new());

    while let Some(request_result) = framed.next().await {
        let bytes = request_result?;

        // 2. Decode using serde_cbor
        let rhex_list: Vec<Rhex> = serde_cbor::from_slice(&bytes)?;
        println!("Received {} items", rhex_list.len());

        let response = serde_cbor::to_vec(&rhex_list)?;

        // 4. Send back through the frame
        framed.send(response.into()).await?;
    }

    Ok(())
}
