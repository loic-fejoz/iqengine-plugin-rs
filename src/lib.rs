#![feature(array_chunks)]
#![feature(iter_array_chunks)]
#[macro_use]
extern crate serde_derive;

extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate url;

pub mod server;
