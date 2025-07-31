use clap::{Parser, Subcommand};
use ossa_core::util::TypedStream;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub(crate) struct Arguments {
    #[arg(short, long, value_name = "PORT")]
    pub(crate) port: Option<u16>,
}

// // TMP for testing
// use odyssey_core::network::protocol::run_store_metadata_client;
// use odyssey_core::protocol::v0::StoreMetadataHeaderRequest;
// use odyssey_core::util::Sha256Hash;
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
