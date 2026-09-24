use std::sync::Arc;

use anyhow::Result;
use futures::{SinkExt, StreamExt};
use iam::IAm;
use key::enclave::Enclave;
use lattice::{Lattice, Rhex};
use tokio::sync::RwLock;
use transform::registry::TransformRegistry;

use crate::{
    config::UsherdConfig,
    rebuild,
    receive::{ReceiveStatus, receive},
};

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
    let mut enclave = Enclave::new(Some(config.enclave.clone()));
    enclave.populate()?;

    // RwLock-ed items
    let lattice = Arc::new(RwLock::new(lattice));
    let trans_registry = Arc::new(RwLock::new(trans_registry));
    let iam = Arc::new(RwLock::new(iam));
    let enclave = Arc::new(RwLock::new(enclave));

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    println!("🟢 Server listening on {}", addr);

    loop {
        let (stream, _) = listener.accept().await?;
        let c = config.clone();
        let lattice_clone = Arc::clone(&lattice);
        let trans_reg_clone = Arc::clone(&trans_registry);
        let iam_clone = Arc::clone(&iam);
        let enclave_clone = Arc::clone(&enclave);
        tokio::spawn(async move {
            if let Err(e) = handle_connection(
                stream,
                c,
                lattice_clone,
                trans_reg_clone,
                iam_clone,
                enclave_clone,
            )
            .await
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
    enclave: Arc<RwLock<Enclave>>,
) -> Result<()> {
    // We use LengthDelimitedCodec so we don't have to worry about
    // TCP fragmenting our CBOR blobs.
    let mut framed =
        tokio_util::codec::Framed::new(stream, tokio_util::codec::LengthDelimitedCodec::new());

    while let Some(request_result) = framed.next().await {
        let bytes = request_result?;
        let mut output = Vec::new();

        // 2. Decode using minicbor
        let rhex_list: Vec<Rhex> = minicbor::decode(&bytes)?;
        println!("Received {} items", rhex_list.len());

        {
            let mut lattice_guard = lattice.write().await;
            let mut trans_reg_guard = trans_registry.write().await;
            let mut iam_guard = iam.write().await;
            let mut enclave_guard = enclave.write().await;

            for rhex in &rhex_list {
                // Append rhex here
                let receive_output = receive(
                    &config,
                    rhex,
                    &mut trans_reg_guard,
                    &mut lattice_guard,
                    &mut iam_guard,
                    &mut enclave_guard,
                )?;
                match receive_output.0 {
                    ReceiveStatus::FailedValidation(_) => {
                        // TODO: build failed validation R⬢ to return to client
                    }
                    ReceiveStatus::MissingSignature(_) => {
                        // TODO: build failed validation R⬢ to return to client
                    }
                    // success means we have output from the append process
                    // already to go, so we just have to attach it to the
                    // output.
                    ReceiveStatus::Success => {
                        let mut out = receive_output.1.unwrap();
                        output.append(&mut out);
                    }
                }
            }
        }
        let mut response = Vec::new();
        minicbor::encode(&output, &mut response)?;

        // 4. Send back through the frame
        framed.send(response.into()).await?;
    }

    Ok(())
}
