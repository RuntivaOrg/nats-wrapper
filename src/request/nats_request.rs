use tonic::metadata::{AsciiMetadataKey, KeyAndValueRef, MetadataMap};

use crate::proto;

// TryFrom converter used for NatsRequest to extract both generic data and the MetadataMap headers
pub trait TryFromNatsRequest<T>: Sized {
    /// The type returned in the event of a conversion error.
    type Error;

    /// Performs the conversion.
    fn try_from(value: T) -> Result<(Self, RequestHeaders), Self::Error>;
}

/// Wrapper for MetadataMap to allow us to implement From<proto::MetadataMap> for tonic::metadata::MetadataMap (as they are both defined in different crates)
#[derive(Debug)]
pub struct RequestHeaders(pub MetadataMap);

impl RequestHeaders {
    pub fn new() -> Self {
        Self(MetadataMap::new())
    }
}

#[derive(Debug)]
pub struct NatsEnvelope<T> {
    pub headers: RequestHeaders,
    pub data: T,
}

impl<T> NatsEnvelope<T> {
    pub fn new(headers: RequestHeaders, data: T) -> Self {
        Self { headers, data }
    }
}

// Converter for proto::MetadataMap to tonic::metadata::MetadataMap
// This is used in the TryFromNatsRequest<T> implementations to extract out the headers
impl From<Vec<proto::MetadataMap>> for RequestHeaders {
    fn from(values: Vec<proto::MetadataMap>) -> Self {
        let mut map = MetadataMap::new();
        for header_val in values.iter() {
            let key = header_val.key.as_bytes();
            let values = header_val.value.iter();
            let mut ctr = 0;
            for value in values {
                ctr += 1;
                if ctr == 1 {
                    map.insert(
                        AsciiMetadataKey::from_bytes(key).unwrap(),
                        value.parse().unwrap(),
                    );
                } else {
                    map.append(
                        AsciiMetadataKey::from_bytes(key).unwrap(),
                        value.parse().unwrap(),
                    );
                }
            }
        }

        RequestHeaders(map)
    }
}

// Converter for tonic::metadata::MetadataMap to proto::MetadataMap
// This is used in generated gRPC-based NATs requests
impl From<RequestHeaders> for Vec<proto::MetadataMap> {
    fn from(value: RequestHeaders) -> Self {
        let mut headers: Vec<proto::MetadataMap> = vec![];

        for key_and_value in value.0.iter() {
            match key_and_value {
                KeyAndValueRef::Ascii(ref key, _) => {
                    let k = key.to_string();
                    let view = value.0.get_all(&k);

                    // only add the key once...
                    if !headers.iter().any(|h| h.key == k) {
                        let v: Vec<String> = view
                            .iter()
                            .map(|v| v.to_str().unwrap().to_string())
                            .collect::<Vec<_>>();

                        headers.push(proto::MetadataMap {
                            key: key.to_string(),
                            value: v,
                        })
                    }
                }
                _ => (),
            }
        }

        headers
    }
}

#[cfg(test)]
#[path = "./nats_request_tests.rs"]
mod nats_request_tests;
