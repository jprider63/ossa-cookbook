use clap::{Parser};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub(crate) struct Arguments {
    #[arg(short, long, value_name = "PORT", default_value_t=8080, help = "Port to run the Ossa server on. If the port is already in use, the next available one will be used.")]
    pub(crate) port: u16,
    #[arg(long, help = "Attempt NAT traversal using UPnP IGD.")]
    pub(crate) nat_traversal: bool,
}

// // TMP for testing
// use ossa_core::network::protocol::run_store_metadata_client;
// use ossa_core::protocol::v0::StoreMetadataHeaderRequest;
// use ossa_core::util::Sha256Hash;
// use tokio::net::TcpStream;
// use tokio_util::codec::{self, LengthDelimitedCodec};
// pub(crate) fn run_client() {
//     // TODO: DELETE ME XXX
//     tokio::runtime::Runtime::new().unwrap().block_on(async {
//         let tcpstream = TcpStream::connect("127.0.0.1:9999")
//             .await
//             .expect("Failed to connect to server");
//         let stream = codec::Framed::new(tcpstream, LengthDelimitedCodec::new());
//         let mut stream = TypedStream::new(stream);
//
//         let req = StoreMetadataHeaderRequest {
//             store_id: Sha256Hash([0; 32]),
//             body_request: None,
//         };
//         let response = run_store_metadata_client::<Sha256Hash, _>(&mut stream, req)
//             .await
//             .unwrap();
//
//         println!("Recieved: {:?}", response);
//     })
// }
