use anyhow::Result;
use futures::{SinkExt, StreamExt};
use lattice::Rhex;

use crate::{config::UsherdConfig, rebuild};

pub async fn run(config: UsherdConfig) -> Result<()> {
    let addr = format!("0.0.0.0:{}", config.port);

    // If rebuild=true we fire off the rebuilt bootstrap procedure,
    // otherwise we build from our existing cache

    let lattice = if config.rebuild {
        rebuild::rebuild(&config).unwrap()
    } else {
        let mut building_lattice = lattice::Lattice::new();
        building_lattice.startup(&config.scopes)?;
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

        // 2. Decode using minicbor
        let rhex_list: Vec<Rhex> = minicbor::decode(&bytes)?;
        println!("Received {} items", rhex_list.len());

        for _rhex in &rhex_list {
            // Append rhex here
        }
        let mut response = Vec::new();
        minicbor::encode(&rhex_list, &mut response)?;

        // 4. Send back through the frame
        framed.send(response.into()).await?;
    }

    Ok(())
}
