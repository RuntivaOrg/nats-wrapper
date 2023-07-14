#![deny(unsafe_code, unused_qualifications, trivial_casts)]
#![deny(clippy::all)]
// #![warn(clippy::pedantic)]
/// This library wraps the [async-nats] crate adding centralized functionality
/// of serializion (gRPC and JSON) and applying consistent message wrapper and metadata
///
/// It also provides a response object used in the reply portion of a NATS request/reply call.
///
pub mod reply;
pub mod request;
pub mod server;
mod subject;
pub use subject::SubjectName;

#[allow(unused_qualifications)]
#[allow(clippy::all)]
pub mod proto_test {
    tonic::include_proto!("test");

    pub const FILE_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("test");
}
