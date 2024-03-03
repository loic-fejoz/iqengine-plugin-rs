#![feature(array_chunks)]
#![feature(iter_array_chunks)]
// #![feature(async_fn_in_trait)]

#[macro_use]
extern crate serde_derive;

extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate url;

pub mod server;
