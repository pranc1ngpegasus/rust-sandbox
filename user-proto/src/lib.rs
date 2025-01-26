pub mod types {
    pub use prost_types::Timestamp;
}

pub mod user {
    pub mod v1 {
        include!(concat!(env!("OUT_DIR"), "/user.v1.rs"));
        pub const FILE_DESCRIPTOR_SET: &[u8] =
            tonic::include_file_descriptor_set!("file_descriptor");
    }
}
