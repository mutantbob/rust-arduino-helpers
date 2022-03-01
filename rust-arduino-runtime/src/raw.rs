#![allow(non_snake_case, non_upper_case_globals, non_camel_case_types)]
#![allow(dead_code)]
#![allow(clippy::all)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub mod ip_address {
    use crate::raw::Print;

    include!(concat!(env!("OUT_DIR"), "/bindings_ipaddress.rs"));
}

pub mod client {
    include!(concat!(env!("OUT_DIR"), "/bindings_client.rs"));
}

pub mod stream {
    include!(concat!(env!("OUT_DIR"), "/bindings_stream.rs"));
}
