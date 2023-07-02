mod nats_server;
pub use nats_server::NatsServer;

mod server_traits;
pub use server_traits::{PublishJson, PublishProst, RequestJson, RequestProst};

pub mod receiver;

mod error;
pub use error::NatsWrapperError;

pub mod serde;

#[allow(unused_qualifications)]
#[allow(clippy::all)]
pub mod proto {
    tonic::include_proto!("test");

    pub const FILE_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("test");
}
