use std::sync::Arc;

use anyhow::Result;
use futures::{SinkExt, StreamExt};
use iam::IAm;
use lattice::{Lattice, Rhex};
use tokio::sync::RwLock;
use transform::registry::TransformRegistry;

use crate::{config::UsherdConfig, rebuild, receive::receive};

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
    // TODO: Load Transforms into registry
    let trans_registry = TransformRegistry::new();
    // TODO: Load IAm entries
    let iam = IAm::new();

    // RwLock-ed items
    let lattice = Arc::new(RwLock::new(lattice));
    let trans_registry = Arc::new(RwLock::new(trans_registry));
    let iam = Arc::new(RwLock::new(iam));

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    println!("🟢 Server listening on {}", addr);

    loop {
        let (stream, _) = listener.accept().await?;
        let c = config.clone();
        let lattice_clone = Arc::clone(&lattice);
        let trans_reg_clone = Arc::clone(&trans_registry);
        let iam_clone = Arc::clone(&iam);
        tokio::spawn(async move {
            if let Err(e) =
                handle_connection(stream, c, lattice_clone, trans_reg_clone, iam_clone).await
            {
                eprintln!("Connection error: {:?}", e);
            }
        });
    }
}

async fn handle_connection(
    stream: tokio::net::TcpStream,
    config: UsherdConfig,
    lattice: Arc<RwLock<Lattice>>,
    trans_registry: Arc<RwLock<TransformRegistry>>,
    iam: Arc<RwLock<IAm>>,
) -> Result<()> {
    // We use LengthDelimitedCodec so we don't have to worry about
    // TCP fragmenting our CBOR blobs.
    let mut framed =
        tokio_util::codec::Framed::new(stream, tokio_util::codec::LengthDelimitedCodec::new());

    while let Some(request_result) = framed.next().await {
        let bytes = request_result?;

        // 2. Decode using minicbor
        let rhex_list: Vec<Rhex> = minicbor::decode(&bytes)?;
        println!("Received {} items", rhex_list.len());

        {
            let mut lattice_guard = lattice.write().await;
            let mut trans_reg_guard = trans_registry.write().await;
            let mut iam_guard = iam.write().await;
            for rhex in &rhex_list {
                // Append rhex here
                let _receive_output = receive(
                    &config,
                    rhex.clone(),
                    &mut trans_reg_guard,
                    &mut lattice_guard,
                    &mut iam_guard,
                );
            }
        }
        let mut response = Vec::new();
        minicbor::encode(&rhex_list, &mut response)?;

        // 4. Send back through the frame
        framed.send(response.into()).await?;
    }

    Ok(())
}
